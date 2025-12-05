use axum::{Router, routing::get};

use crate::AppState;

use super::{beatmapset, search};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search", get(search::search_v2))
        // .route("/beatmaps/{id}", get(beatmaps::get_beatmaps_v2))
        // .route("/beatmaps/md5/{md5}", get(beatmaps::get_beatmaps_by_md5_v2))
        .route("/beatmapsets/{id}", get(beatmapset::get_beatmapset_v2))
}
