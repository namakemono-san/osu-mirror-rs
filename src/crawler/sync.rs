use super::client::{ApiBeatmap, ApiBeatmapset, OsuClient};
use crate::db::models::{Beatmap, Beatmapset};
use crate::db::queries;
use anyhow::Result;
use sqlx::PgPool;

pub async fn sync_beatmapsets_page(
    pool: &PgPool,
    client: &OsuClient,
    query: &str,
    cursor: Option<String>,
) -> Result<Option<String>> {
    tracing::info!("Syncing beatmapsets... query={}", query);

    let response = match client.search_beatmapsets(query, cursor.as_deref()).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("search_beatmapsets failed: {}", e);
            return Err(e);
        }
    };

    tracing::info!("Fetched {} beatmapsets", response.beatmapsets.len());

    for api_beatmapset in response.beatmapsets {
        if let Err(e) = save_beatmapset(pool, api_beatmapset).await {
            tracing::error!("Failed to save beatmapset: {}", e);
        }
    }

    Ok(response.cursor_string)
}

pub async fn save_beatmapset(pool: &PgPool, api_set: ApiBeatmapset) -> Result<()> {
    let creator_id = api_set.user_id;

    let beatmapset = Beatmapset {
        id: api_set.id,
        title: api_set.title,
        title_unicode: api_set.title_unicode,
        artist: api_set.artist,
        artist_unicode: api_set.artist_unicode,
        creator: api_set.creator,

        creator_id: creator_id,
        genre_id: api_set.genre_id,
        language_id: api_set.language_id,
        rating: api_set.rating,

        source: api_set.source,
        tags: api_set.tags,
        status: api_set.status,
        ranked_date: api_set.ranked_date,
        submitted_date: api_set.submitted_date,
        last_updated: api_set.last_updated,
        bpm: api_set.bpm,
        video: api_set.video,
        storyboard: api_set.storyboard,
        nsfw: api_set.nsfw,
        favourite_count: api_set.favourite_count,
        play_count: api_set.play_count,
        availability_download_disabled: api_set
            .availability
            .as_ref()
            .map(|a| a.download_disabled)
            .unwrap_or(false),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        beatmaps: None,
    };

    queries::upsert_beatmapset(pool, &beatmapset).await?;

    if let Some(api_beatmaps) = api_set.beatmaps {
        for api_beatmap in api_beatmaps {
            let beatmap = convert_api_beatmap(api_beatmap);
            queries::upsert_beatmap(pool, &beatmap).await?
        }
    }

    Ok(())
}

fn convert_api_beatmap(api: ApiBeatmap) -> Beatmap {
    Beatmap {
        id: api.id,
        beatmapset_id: api.beatmapset_id,
        version: api.version,
        mode: api.mode,
        mode_int: api.mode_int,
        difficulty_rating: api.difficulty_rating,
        ar: api.ar,
        cs: api.cs,
        drain: api.drain,
        accuracy: api.accuracy,
        bpm: api.bpm,
        total_length: Some(api.total_length),
        hit_length: api.hit_length,
        max_combo: api.max_combo,
        count_circles: api.count_circles,
        count_sliders: api.count_sliders,
        count_spinners: api.count_spinners,
        checksum: api.checksum,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

pub async fn save_cursor(pool: &PgPool, id: &str, cursor: Option<String>) -> Result<()> {
    sqlx::query!(
        r#"INSERT INTO sync_cursors (id, cursor_string, last_sync) VALUES ($1, $2, NOW())
        ON CONFLICT (id) DO UPDATE SET cursor_string = EXCLUDED.cursor_string, last_sync = NOW()"#,
        id,
        cursor
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn load_cursor(pool: &PgPool, id: &str) -> Result<Option<String>> {
    let record = sqlx::query!(
        r#"SELECT cursor_string FROM sync_cursors WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(record.and_then(|r| r.cursor_string))
}
