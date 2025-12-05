use super::docs::openapi_json;
use super::download;
use super::health;
use super::v1;
use super::v2;

use crate::AppState;
use crate::middleware::rate_limit::RateLimitLayer;

use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: &'static str,
}

async fn handle_404() -> Response {
    let body = ErrorResponse {
        error: "Not Found",
        message: "The requested resource was not found",
    };

    (StatusCode::NOT_FOUND, Json(body)).into_response()
}

async fn docs_handler() -> Html<String> {
    Html(
        r#"<!doctype html>
<html>
  <head>
    <title>osu-mirror-rs API Reference</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <div id="app"></div>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
    <script>
      Scalar.createApiReference('#app', {
        url: '/docs/openapi.json',
        defaultOpenAllTags: true,
        showDeveloperTools: "never",
      })
    </script>
  </body>
</html>"#
            .to_string(),
    )
}

pub fn create_router(state: AppState) -> Router {
    let v1_router = v1::routes::router();
    let v2_router = v2::routes::router();

    Router::new()
        .nest("/v1", v1_router)
        .nest("/v2", v2_router)
        // Status
        .route("/health", get(health::health_check))
        .route("/status", get(health::status))
        // Download
        .route("/d/{id}", get(download::download_beatmapsets))
        // Docs
        .route("/docs", get(docs_handler))
        .route("/docs/openapi.json", get(openapi_json))
        // Other
        .fallback(handle_404)
        .layer(RateLimitLayer::new(60, 60))
        .with_state(state)
}
