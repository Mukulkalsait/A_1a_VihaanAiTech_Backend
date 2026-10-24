// FILE: ./src/headers/me_auth.rs

use axum;
use serde_json;

use crate::app;
use crate::errors::ApiError;
use crate::utils::auth::get_authenticated_user;

pub async fn me(axum::extract::State(state): axum::extract::State<app::AppState>, headers: axum::http::HeaderMap) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let user = get_authenticated_user(&headers, &state).await?;

    // convert itto json
    Ok(axum::Json(serde_json::json!({
        "id":user.user_id,
        "email":user.user_email,
        "name":user.user_first_name,
        "picture":user.user_picture,
    })))
}
