use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
            AppError::Storage(e) => {
                tracing::error!("Storage error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Storage error".to_string(),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
            ),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
