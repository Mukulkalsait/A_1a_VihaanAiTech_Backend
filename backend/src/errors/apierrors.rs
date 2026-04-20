use axum;
use serde;
use thiserror;
// EXT:

#[derive(serde::Serialize)]
/// ErrorResponse with serde:Serialize.
/// * This gives Json Like SINGLE ERROR.
/// ```json
/// { error : "Message" }
/// ```
/// > ⚡SINGLE ERROR PER RESPONSE.
struct ErrorResponse {
    error: String,
}

#[derive(Debug, thiserror::Error)]
/// Closed Set Of Error APIs.
/// Varients => DIrect Http Mappings.
/// ThisError => AutoDisplay.
/// ``` rs
/// # Implimentation of axum::response::IntoResponse{
///  fn providing(self) -> returning axum::response::Response
///  let (status,message) =  match self
///   ApiError {all 5 options } =>
///     returning
///     axum::http::StatusCode::same_5_optiosn, ( self/message ).to_string.
///
///  let body = axum::Json(struct ErrorResponse{...});
///
///  (stats,body).into_response => Returning function.
/// }
/// ```
///
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbiden")]
    Forbiden,
    #[error("resource not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error")]
    Internal,
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbiden => (axum::http::StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            ApiError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };
        let body = axum::Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
