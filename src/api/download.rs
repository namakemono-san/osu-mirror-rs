use crate::{
    AppState, crawler,
    db::queries,
    error::{AppError, Result},
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::Response,
};
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Mutex;
use std::time::{Duration, Instant};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("failed to build reqwest client")
});

struct MirrorCache {
    url: Option<String>,
    ts: Instant,
}

static MIRROR_CACHE: Lazy<Mutex<MirrorCache>> = Lazy::new(|| {
    Mutex::new(MirrorCache {
        url: None,
        ts: Instant::now(),
    })
});

#[derive(Deserialize)]
pub struct DownloadParams {
    #[serde(rename = "nv")]
    nv: Option<String>,
    #[serde(rename = "novideo")]
    novideo: Option<String>,
}

fn parse_bool_param(v: &str) -> Option<bool> {
    match v {
        "" => Some(true),
        "1" => Some(true),
        "0" => Some(false),
        "true" | "True" | "TRUE" => Some(true),
        "false" | "False" | "FALSE" => Some(false),
        _ => None,
    }
}

fn parse_no_video(params: &DownloadParams) -> bool {
    if let Some(ref nv) = params.nv {
        if let Some(b) = parse_bool_param(nv) {
            return b;
        }
    }
    if let Some(ref nv) = params.novideo {
        if let Some(b) = parse_bool_param(nv) {
            return b;
        }
    }
    false
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn build_osz_response(data: Bytes, filename: &str, cache_status: &str) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-osu-beatmap-archive")
        .header(
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="{}""#, filename),
        )
        .header("X-Cache-Status", cache_status)
        .body(Body::from(data))
        .unwrap()
}

fn build_mirror_urls(id: i64, no_video: bool) -> Vec<String> {
    let nv = if no_video { "1" } else { "0" };
    let mut urls = Vec::new();
    urls.push(format!("https://api.nerinyan.moe/d/{}?nv={}", id, nv));
    urls.push(format!("https://catboy.best/d/{}?nv={}", id, nv));
    urls.push(format!("https://osu.direct/api/d/{}?nv={}", id, nv));
    let bc_url = if no_video {
        format!("https://beatconnect.io/b/{}?novideo=1", id)
    } else {
        format!("https://beatconnect.io/b/{}", id)
    };
    urls.push(bc_url);
    urls
}

fn is_valid_zip(bytes: &[u8]) -> bool {
    bytes.starts_with(b"PK\x03\x04")
}

async fn try_download_once(url: &str) -> Result<Option<Bytes>> {
    tracing::info!("mirror probe start: {}", url);

    let resp = match HTTP_CLIENT
        .get(url)
        .header("User-Agent", "osu-mirror-rs/1.0")
        .header("Accept", "*/*")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("mirror {} request failed: {}", url, e);
            return Ok(None);
        }
    };

    let status = resp.status();
    tracing::info!("mirror {} returned status {}", url, status);

    if status.as_u16() >= 500 {
        tracing::warn!("mirror {} server error: {}", url, status);
        return Ok(None);
    }

    let body = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!("mirror {} body read failed: {}", url, e);
            return Ok(None);
        }
    };

    if !is_valid_zip(&body) {
        tracing::warn!("mirror {} returned non-zip ({} bytes)", url, body.len());
        return Ok(None);
    }

    tracing::info!("mirror {} download OK ({} bytes)", url, body.len());
    Ok(Some(body))
}

async fn download_from_mirrors(id: i64, no_video: bool) -> Result<Bytes> {
    let mut urls = build_mirror_urls(id, no_video);

    {
        let cache = MIRROR_CACHE.lock().unwrap();
        let ttl = Duration::from_secs(20);
        if let Some(ref cached_url) = cache.url {
            if cache.ts.elapsed() < ttl {
                let mut new_urls = Vec::with_capacity(urls.len());
                new_urls.push(cached_url.clone());
                for u in urls.into_iter() {
                    if u != *cached_url {
                        new_urls.push(u);
                    }
                }
                urls = new_urls;
            }
        }
    }

    for url in urls.iter() {
        if let Some(data) = try_download_once(url).await? {
            let mut cache = MIRROR_CACHE.lock().unwrap();
            cache.url = Some(url.clone());
            cache.ts = Instant::now();
            return Ok(data);
        }
    }

    Err(AppError::Internal(
        "all mirrors failed to provide beatmapset".to_string(),
    ))
}

pub async fn download_beatmapsets(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<DownloadParams>,
) -> Result<Response> {
    let no_video = parse_no_video(&params);
    tracing::info!("download request: {} (no_video: {})", id, no_video);

    let mut set = queries::get_beatmapset(&state.db, id).await?;

    if set.is_none() {
        tracing::info!(
            "beatmapset {} not found locally → fetching from osu! API",
            id
        );

        let api_set = match state.osu_client.get_beatmapset(id).await {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("failed to fetch beatmapset {} from API: {}", id, e);
                return Err(AppError::NotFound(format!("Beatmapset {} not found", id)));
            }
        };

        if let Err(e) = crawler::sync::save_beatmapset(&state.db, api_set).await {
            tracing::error!("failed to upsert beatmapset {} into DB: {}", id, e);
            return Err(AppError::Internal(
                "failed to persist beatmapset metadata".to_string(),
            ));
        }

        set = queries::get_beatmapset(&state.db, id).await?;
    }

    let Some(beatmapset) = set else {
        return Err(AppError::NotFound(format!("Beatmapset {} not found", id)));
    };

    if beatmapset.availability_download_disabled {
        return Err(AppError::NotFound("Download disabled".to_string()));
    }

    let base_name = format!("{} {} - {}", id, beatmapset.artist, beatmapset.title);
    let full_name = if no_video {
        format!("{} [no video].osz", base_name)
    } else {
        format!("{}.osz", base_name)
    };
    let filename = sanitize_filename(&full_name);

    if let Ok(Some(bytes)) = state.storage.get(id, no_video).await {
        tracing::info!("cache HIT: {} (no_video: {})", id, no_video);
        return Ok(build_osz_response(bytes, &filename, "HIT"));
    }

    tracing::info!("cache MISS: {} (no_video: {})", id, no_video);
    let data = download_from_mirrors(id, no_video).await?;

    if let Err(e) = state.storage.put(id, no_video, data.clone()).await {
        tracing::error!("failed to cache beatmapset {}: {}", id, e);
    }

    let storage_path = format!("{}/{}.osz", id / 1000, id);
    let backend = format!("{:?}", state.config.storage.backend).to_lowercase();

    let _ = sqlx::query!(
        r#"
        INSERT INTO cache_metadata (
            beatmapset_id,
            file_size,
            storage_path,
            storage_backend,
            no_video
        )
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (beatmapset_id)
        DO UPDATE SET
            last_accessed = NOW(),
            file_size = EXCLUDED.file_size,
            storage_path = EXCLUDED.storage_path,
            storage_backend = EXCLUDED.storage_backend,
            no_video = EXCLUDED.no_video
        "#,
        id,
        data.len() as i64,
        storage_path,
        backend,
        no_video
    )
    .execute(&state.db)
    .await;

    Ok(build_osz_response(data, &filename, "MISS"))
}
