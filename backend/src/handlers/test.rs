// FILE: ./src/handlers/test.rs

use axum;
// EXT:
use crate::app::AppState;
use crate::errors::ApiError;
// INT:

pub async fn fail(axum::extract::State(state): axum::extract::State<AppState>) -> Result<&'static str, ApiError> {
    println!("ENV = {:?}", state.config.app_env);
    println!("FORBIDEN HIT...");
    Err(ApiError::Unauthorized)
}
pub async fn forbiden() -> Result<&'static str, ApiError> {
    println!("FORBIDEN HIT...");
    Err(ApiError::Forbidden)
}
