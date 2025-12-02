use crate::AppState;
use axum::{extract::State, Json};
use serde_json::json;

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    Json(json!({
        "status": "running",
        "database": if db_status { "connected" } else { "error" },
        "storage_backend": format!("{:?}", state.config.storage.backend),
    }))
}
