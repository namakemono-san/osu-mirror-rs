use axum::{
    Json,
    extract::{Path, State},
};

use crate::{AppState, db::queries, error::Result};

use super::mapping::{BeatmapsetV2, map_set_v2};

pub async fn get_beatmapset_v2(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Option<BeatmapsetV2>>> {
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
                return Ok(Json(None));
            }
        };

        if let Err(e) = crate::crawler::sync::save_beatmapset(&state.db, api_set).await {
            tracing::error!("Failed to upsert beatmapset {} into DB: {}", id, e);
            return Ok(Json(None));
        }

        set = queries::get_beatmapset(&state.db, id).await?;
    }

    let Some(set) = set else {
        return Ok(Json(None));
    };

    Ok(Json(Some(map_set_v2(set))))
}
