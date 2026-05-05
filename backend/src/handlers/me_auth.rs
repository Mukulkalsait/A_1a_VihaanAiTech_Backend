use crate::{app, errors::ApiError};
use axum;
use serde_json;

pub async fn me(
    axum::extract::State(state): axum::extract::State<app::AppState>,
    headers: axum::http::HeaderMap,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // 1. Get auth error
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or(ApiError::Unauthorized)?;
    // 2. Extract Token
    let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
    // 3. Decode JWT
    let decoded = jsonwebtoken::decode::<crate::utils::jwt::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_ref()), // Appstate->AppConfig->jwt_secret
        &jsonwebtoken::Validation::default(),                                      // default validation
    )
    .map_err(|_| ApiError::Unauthorized)?;
    // 4. Fetch User.
    let user_id = decoded.claims.sub; // sub => who this token billongs to. Y: sub= user.id
    let user = sqlx::query!(r#" SELECT id as "id!: i64", email, first_name, picture FROM core_users WHERE id = ?"#, user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(axum::Json(serde_json::json!({
        "id":user.id,
        "email":user.email,
        "name":user.first_name,
        "picture":user.picture,
    })))
}
