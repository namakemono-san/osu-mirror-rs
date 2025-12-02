use super::{beatmap, beatmapset, search};
use crate::AppState;
use axum::{Router, routing::get};

pub fn router(state: AppState) -> Router {
    Router::new()
    // .route("/search", get(search::search_v2))
    // .route("/beatmaps/{id}", get(beatmaps::get_beatmaps_v2))
    // .route("/beatmaps/md5/{id}", get(beatmaps::get_beatmaps_by_md5_v2))
    // .route("/beatmapsets/{id}", get(beatmapset::get_beatmapset_v2))
}
