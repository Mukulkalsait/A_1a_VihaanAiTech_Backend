use crate::app;
use axum::{self, http};
use serde_json;

pub async fn me(axum::extract::State(state): State<app::AppState>, headers: axum::http::HeaderMap) {
    // 1. Get auth error
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());
    // 2. Extract Token
    let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
    // 3. Decode JWT
    let decoded = jsonwebtoken::decode::<crate::utils::jwt::Claim>(token, 
        &jsonwebtoken::DecodingKey::from_secret(state,config.jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),)
    // 4. Fetch User.
}
