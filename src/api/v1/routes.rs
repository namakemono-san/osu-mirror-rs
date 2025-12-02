use axum::{Router, routing::get};

use crate::AppState;

use super::{beatmaps, beatmapset, search};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search", get(search::search_v1))
        .route("/beatmaps/{id}", get(beatmaps::get_beatmaps_v1))
        .route("/beatmaps/md5/{md5}", get(beatmaps::get_beatmaps_by_md5_v1))
        .route("/beatmapsets/{id}", get(beatmapset::get_beatmapset_v1))
}
