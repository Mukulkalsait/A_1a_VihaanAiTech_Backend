// FILE: ./src/headers/me_auth.rs

use crate::{app, errors::ApiError};
use axum;
use serde_json;

/// ## Step 1: Client sends request
/// ```http GET /me
/// Authorization: Bearer <JWT_TOKEN>
/// ```
/// ## Step 2: Key Code Explanations
/// ```rust
/// let auth_header = headers.get("authorization") //  get token from request
/// let token = auth_header.strip_prefix("Bearer ") // remove "Bearer "
/// jsonwebtoken::decode(...) // verify token is valid
/// let user_id = decoded.claims.sub; // extract user ID from token
/// SELECT * FROM core_users WHERE id = ? // fetch user
/// return JSON
/// ```
/// ## FINAL RESULT
/// ```json
/// { "id": 1, "email": "...", "name": "...", "picture": "..." }
/// ```
/// ## WorkFlow Reason:
///  - Frontend already has token
///  - asks backend: "who is logged in?"
///
/// # CONCEPTS:
/// JWT from frontend has 3 parts,
///  1.Header | 2.Payload | 3.Signature
///  - Signature => proof out backend created thsi tooken.
///  - this is created by that EncodingKey::from_secret(...) in [jwt.rs](../utils/jwt.rs) file.
///
pub async fn me(
    axum::extract::State(state): axum::extract::State<app::AppState>,
    headers: axum::http::HeaderMap,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // 1. Get auth error
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or(ApiError::Unauthorized)?;
    // 2. Extract Token
    let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
    // 3. Decode JWT SERDE DECODIGN TECHNIWUE <TURBO FISH>
    let decoded = jsonwebtoken::decode::<crate::utils::jwt::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_ref()), // Appstate->AppConfig->jwt_secret
        &jsonwebtoken::Validation::default(),                                      // default validation
    )
    .map_err(|_| ApiError::Unauthorized)?;
    // 4. Fetch User.
    let user_id = decoded.claims.sub; // sub => who this token billongs to. Y: sub= user.id
    let user = sqlx::query!(
        r#"
        SELECT 
            user_id as "id!: i64",
            user_email,
            user_first_name,
            user_picture
            FROM core_users
            WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| ApiError::Internal)?;

    let user = user.ok_or(ApiError::Unauthorized);

    Ok(axum::Json(serde_json::json!({
        "id":user.as_ref().unwrap().id,
        "email":user.as_ref().unwrap().user_email,
        "name":user.as_ref().unwrap().user_first_name,
        "picture":user.as_ref().unwrap().user_picture,
    })))
}
