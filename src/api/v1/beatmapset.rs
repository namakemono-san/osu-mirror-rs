use axum::{
    Json,
    extract::{Path, State},
};

use crate::{AppState, db::queries, error::Result};

use super::mapping::BeatmapV1;

pub async fn get_beatmapset_v1(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<BeatmapV1>>> {
    let mut set = queries::get_beatmapset(&state.db, id).await?;

    if set.is_none() {
        tracing::info!(
            "Beatmapset {} not found locally → fetching from osu! API",
            id
        );

        let api_set = match state.osu_client.get_beatmapset(id).await {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Failed to fetch beatmapset {} from API: {}", id, e);
                return Ok(Json(Vec::new()));
            }
        };

        if let Err(e) = crate::crawler::sync::save_beatmapset(&state.db, api_set).await {
            tracing::error!("Failed to upsert beatmapset {} into DB: {}", id, e);
            return Ok(Json(Vec::new()));
        }

        set = queries::get_beatmapset(&state.db, id).await?;
    }

    let Some(set) = set else {
        return Ok(Json(Vec::new()));
    };

    let mut result = Vec::new();

    if let Some(beatmaps) = set.beatmaps.as_ref() {
        for m in beatmaps {
            result.push(BeatmapV1::from_models(&set, m));
        }
    }

    Ok(Json(result))
}
