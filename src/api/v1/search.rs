use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{AppState, db::queries, error::Result};

use super::mapping::BeatmapV1;

#[derive(Deserialize)]
pub struct SearchParams {
    #[serde(rename = "q", default)]
    query: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    50
}

pub async fn search_v1(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<BeatmapV1>>> {
    let beatmapsets = queries::search_beatmapsets(
        &state.db,
        &params.query,
        params.status.as_deref(),
        params.limit.min(100),
        params.offset,
    )
    .await?;

    let mut result = Vec::new();

    'outer: for s in beatmapsets {
        if let Some(full) = queries::get_beatmapset(&state.db, s.id).await? {
            if let Some(beatmaps) = full.beatmaps.as_ref() {
                for m in beatmaps {
                    result.push(BeatmapV1::from_models(&full, m));
                    if result.len() as i64 >= params.limit {
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(Json(result))
}
