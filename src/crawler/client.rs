use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use tokio::time::Duration;

static RATE_LIMIT: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(50));

pub async fn start_rate_limiter() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let current = RATE_LIMIT.available_permits();
            if current < 50 {
                RATE_LIMIT.add_permits(50 - current);
            }
        }
    });
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub beatmapsets: Vec<ApiBeatmapset>,
    pub cursor_string: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiBeatmapset {
    pub id: i64,
    pub title: String,
    pub title_unicode: Option<String>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub creator: String,
    pub user_id: Option<i64>,
    pub source: Option<String>,
    pub tags: Option<String>,
    pub status: String,
    pub ranked_date: Option<chrono::DateTime<chrono::Utc>>,
    pub submitted_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub bpm: Option<f64>,
    #[serde(default)]
    pub video: bool,
    #[serde(default)]
    pub storyboard: bool,
    #[serde(default)]
    pub nsfw: bool,
    pub favourite_count: i32,
    pub play_count: i32,
    pub genre_id: Option<i32>,
    pub language_id: Option<i32>,
    pub rating: Option<f64>,
    #[serde(default)]
    pub availability: Option<Availability>,
    pub beatmaps: Option<Vec<ApiBeatmap>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Availability {
    pub download_disabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiBeatmap {
    pub id: i64,
    pub beatmapset_id: i64,
    pub version: String,
    pub mode: String,
    pub mode_int: i32,
    #[serde(default)]
    pub difficulty_rating: Option<f64>,
    #[serde(default)]
    pub ar: Option<f64>,
    #[serde(default)]
    pub cs: Option<f64>,
    pub drain: Option<f64>,
    pub accuracy: Option<f64>,
    pub bpm: Option<f64>,
    pub total_length: i32,
    pub hit_length: Option<i32>,
    pub max_combo: Option<i32>,
    pub count_circles: Option<i32>,
    pub count_sliders: Option<i32>,
    pub count_spinners: Option<i32>,
    pub checksum: Option<String>,
}

pub struct OsuClient {
    client: Client,
    client_id: String,
    client_secret: String,
    token: Arc<RwLock<Option<(String, Instant)>>>,
}

use std::time::Instant;

impl OsuClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            token: Arc::new(RwLock::new(None)),
        }
    }

    async fn authenticate(&self) -> Result<(String, Instant)> {
        let _p = RATE_LIMIT.acquire().await.unwrap();

        let response = self
            .client
            .post("https://osu.ppy.sh/oauth/token")
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "grant_type": "client_credentials",
                "scope": "public"
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("Authentication failed ({}): {}", status, text);
        }

        let tr: TokenResponse = response.json().await?;
        let expiry = Instant::now() + Duration::from_secs(tr.expires_in as u64);

        Ok((tr.access_token, expiry))
    }

    pub async fn ensure_token(&self) -> Result<String> {
        {
            let t = self.token.read().await;
            if let Some((tok, exp)) = t.as_ref() {
                if Instant::now() < *exp {
                    return Ok(tok.clone());
                }
            }
        }

        let (new_token, expiry) = self.authenticate().await?;
        let mut t = self.token.write().await;
        *t = Some((new_token.clone(), expiry));

        Ok(new_token)
    }

    pub async fn search_beatmapsets(
        &self,
        query: &str,
        cursor: Option<&str>,
    ) -> Result<SearchResponse> {
        let query_s = urlencoding::encode(query);

        let mut url = format!("https://osu.ppy.sh/api/v2/beatmapsets/search?q={}", query_s);

        if let Some(c) = cursor {
            url.push_str(&format!("&cursor_string={}", urlencoding::encode(c)));
        }

        let token = self.ensure_token().await?;
        let _p = RATE_LIMIT.acquire().await.unwrap();

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Search failed: {}", resp.status());
        }

        Ok(resp.json().await?)
    }

    pub async fn get_beatmapset(&self, id: i64) -> Result<ApiBeatmapset> {
        let url = format!("https://osu.ppy.sh/api/v2/beatmapsets/{}", id);
        let token = self.ensure_token().await?;
        let _p = RATE_LIMIT.acquire().await.unwrap();

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Get beatmapset failed: {}", resp.status());
        }

        Ok(resp.json().await?)
    }
}
