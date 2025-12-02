use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;

use crate::{AppState, db::models::Beatmap, db::queries, error::Result};

use super::mapping::BeatmapV1;

pub async fn get_beatmaps_v1(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<BeatmapV1>>> {
    let row = sqlx::query!(
        r#"
        SELECT
            id,
            beatmapset_id,
            version,
            mode,
            mode_int,
            difficulty_rating,
            ar,
            cs,
            drain,
            accuracy,
            bpm,
            total_length,
            hit_length,
            max_combo,
            count_circles,
            count_sliders,
            count_spinners,
            checksum,
            created_at,
            updated_at
        FROM beatmaps
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?;

    let Some(r) = row else {
        return Ok(Json(Vec::new()));
    };

    let map = Beatmap {
        id: r.id,
        beatmapset_id: r.beatmapset_id,
        version: r.version,
        mode: r.mode,
        mode_int: r.mode_int,
        difficulty_rating: r.difficulty_rating,
        ar: r.ar,
        cs: r.cs,
        drain: r.drain,
        accuracy: r.accuracy,
        bpm: r.bpm,
        total_length: r.total_length,
        hit_length: r.hit_length,
        max_combo: r.max_combo,
        count_circles: r.count_circles,
        count_sliders: r.count_sliders,
        count_spinners: r.count_spinners,
        checksum: r.checksum,
        created_at: r.created_at.unwrap_or_else(Utc::now),
        updated_at: r.updated_at.unwrap_or_else(Utc::now),
    };

    let set = queries::get_beatmapset(&state.db, map.beatmapset_id).await?;

    let Some(set) = set else {
        return Ok(Json(Vec::new()));
    };

    let v1 = BeatmapV1::from_models(&set, &map);

    Ok(Json(vec![v1]))
}

pub async fn get_beatmaps_by_md5_v1(
    State(state): State<AppState>,
    Path(md5): Path<String>,
) -> Result<Json<Vec<BeatmapV1>>> {
    let row = sqlx::query!(
        r#"
        SELECT
            id,
            beatmapset_id,
            version,
            mode,
            mode_int,
            difficulty_rating,
            ar,
            cs,
            drain,
            accuracy,
            bpm,
            total_length,
            hit_length,
            max_combo,
            count_circles,
            count_sliders,
            count_spinners,
            checksum,
            created_at,
            updated_at
        FROM beatmaps
        WHERE checksum = $1
        LIMIT 1
        "#,
        md5
    )
    .fetch_optional(&state.db)
    .await?;

    let Some(r) = row else {
        return Ok(Json(Vec::new()));
    };

    let map = Beatmap {
        id: r.id,
        beatmapset_id: r.beatmapset_id,
        version: r.version,
        mode: r.mode,
        mode_int: r.mode_int,
        difficulty_rating: r.difficulty_rating,
        ar: r.ar,
        cs: r.cs,
        drain: r.drain,
        accuracy: r.accuracy,
        bpm: r.bpm,
        total_length: r.total_length,
        hit_length: r.hit_length,
        max_combo: r.max_combo,
        count_circles: r.count_circles,
        count_sliders: r.count_sliders,
        count_spinners: r.count_spinners,
        checksum: r.checksum,
        created_at: r.created_at.unwrap_or_else(Utc::now),
        updated_at: r.updated_at.unwrap_or_else(Utc::now),
    };

    let set = queries::get_beatmapset(&state.db, map.beatmapset_id).await?;

    let Some(set) = set else {
        return Ok(Json(Vec::new()));
    };

    let v1 = BeatmapV1::from_models(&set, &map);

    Ok(Json(vec![v1]))
}
