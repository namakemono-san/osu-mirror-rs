use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use super::mapping::{SearchMetaV2, SearchResponseV2, map_set_v2};
use crate::{AppState, db::queries, error::Result};

#[derive(Deserialize)]
pub struct SearchV2Params {
    #[serde(rename = "q", default)]
    pub query: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

pub async fn search_v2(
    State(state): State<AppState>,
    Query(params): Query<SearchV2Params>,
) -> Result<Json<SearchResponseV2>> {
    let total =
        queries::count_beatmapsets(&state.db, &params.query, params.status.as_deref()).await?;

    let beatmapsets = queries::search_beatmapsets(
        &state.db,
        &params.query,
        params.status.as_deref(),
        params.limit.min(100),
        params.offset,
    )
    .await?;

    let mut mapped_sets = Vec::new();

    for s in beatmapsets {
        if let Some(full) = queries::get_beatmapset(&state.db, s.id).await? {
            mapped_sets.push(map_set_v2(full));
            if mapped_sets.len() as i64 >= params.limit {
                break;
            }
        }
    }

    let search_meta = SearchMetaV2 {
        sort: "ranked_desc".to_string(),
    };

    let response = SearchResponseV2 {
        beatmapsets: mapped_sets,
        search: search_meta,
        recommended_difficulty: None,
        error: None,
        total,
        cursor: None,
        cursor_string: None,
    };

    Ok(Json(response))
}
