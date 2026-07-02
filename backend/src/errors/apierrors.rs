use axum;
use serde;
use thiserror;

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error")]
    Internal,
    #[error("internal server error: {0}")] // ← NEW: Add this variant
    InternalWithMessage(String),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbidden => (axum::http::StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            ApiError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            ApiError::InternalWithMessage(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = axum::Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
