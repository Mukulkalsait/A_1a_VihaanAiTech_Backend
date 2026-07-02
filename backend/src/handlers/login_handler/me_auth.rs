// FILE: ./src/headers/me_auth.rs

use crate::{app, errors::ApiError, utils::auth::get_authenticated_user};
use axum;
use serde_json;

pub async fn me(
    axum::extract::State(state): axum::extract::State<app::AppState>,
    headers: axum::http::HeaderMap,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let user = get_authenticated_user(&headers, &state).await?;

    // convert itto json
    Ok(axum::Json(serde_json::json!({
        "id":user.user_id,
        "email":user.user_email,
        "name":user.user_first_name,
        "picture":user.user_picture,
    })))
}
