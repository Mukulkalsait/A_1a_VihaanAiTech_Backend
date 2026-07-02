use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use crate::{app::AppState, errors::ApiError, modals::users_mod};
use jsonwebtoken;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser{
    pub user_id: i64, 
    pub user_email: String, 
    pub user_first_name: String,
    pub user_picture: Option<String>,
}


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
pub async fn get_authenticated_user(headers:&HeaderMap, state:&AppState)-> Result<AuthenticatedUser, ApiError>{
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or(ApiError::Unauthorized)?;
    let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;

    let decoded = jsonwebtoken::decode::<crate::utils::jwt::Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_ref()), // AppState>AppConfig>jwt secret
        &jsonwebtoken::Validation::default(),
    ).map_err(|e| { tracing::error!("Faild to decode the jsonwebtoken: {}",e); ApiError::Unauthorized })?;

    let user_id = decoded.claims.sub; // sub (subject) => who this token billongs ot: SUB=>user.id 
    let user = sqlx::query_as::<_,users_mod::ExcitingUser>(
        r#"
            SELECT 
                user_id as id,
                user_email,
                user_first_name,
                user_picture
            FROM core_users
            WHERE user_id = ? 
        "#,
    ).bind(user_id).fetch_optional(&state.db).await.map_err(|e| {tracing::error!("Faild to run query :{}",e); ApiError::Internal })?.ok_or(ApiError::Unauthorized)?;

    Ok(AuthenticatedUser{
        user_id: user.id,
        user_email: user.user_email,
        user_first_name: user.user_first_name,
        user_picture: user.user_picture
    })

}
