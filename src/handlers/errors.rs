use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
// use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User Not Found: {0} - {1}")]
    UserNotFound(i32, String),
    #[error("Invalid Input: {0} - {1}")]
    InvalidInput(i32, String),
    #[error("Internal server error")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserNotFound(id, msg) => {
                (StatusCode::NOT_FOUND, format!("User not found: {} - {}", id, msg))
            }
            AppError::InvalidInput(id, msg) => {
                (StatusCode::BAD_REQUEST, format!("Invalid input: {} - {}", id, msg))
            }
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", msg))
            }
        };

        (status, Json(serde_json::json!({"error": error_message}))).into_response()
    }
}