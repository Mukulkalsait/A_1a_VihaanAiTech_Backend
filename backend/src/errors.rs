
use axum::{http::StatusCode, response::{ IntoResponse,Response },Json};
use serde::Serialize;
use thiserror::Error;

// --------------------------------------------------------------
#[derive(Debug,Error)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbiden")]
    Forbiden,

    #[error("resourse not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error")]
    Internal,
}

// enum => close set of error APIs
// varietnts => direct http mapping
// BadRequest => Costume messages.
// thierror => AutoDisplay. 
// --------------------------------------------------------------


// --------------------------------------------------------------
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
// this gives json like errors { error: "messages" }
// --------------------------------------------------------------



// --------------------------------------------------------------
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbiden => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };
        let body = Json(ErrorResponse {error:message});
        (status, body).into_response()
    }
}
// IntoResponse creates Resualt<T, E> which can handle by axum automaticaly
// so all we have to do is  Err(ApiError::Unauthorized) -> and it will work.
//  axum will set http code -> stealise jsno -> send response no boilerplate.
// --------------------------------------------------------------
