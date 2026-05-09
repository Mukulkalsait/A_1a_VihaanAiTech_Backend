// FILE: ./src/handlers/google_auth.rs

// use anyhow;
use axum;
use reqwest;
use serde_json;
// use std::fmt::format;
// EXT:

use crate::app::AppState;
use crate::errors::ApiError;
use crate::handlers::user_stractures;
use crate::utils;

/// ## Step 1: Client sends Google token
/// ```json => POST /auth/google
/// { "token": "google_id_token" }
/// ```
/// ## Step 2: Your backend
/// ```rust
/// reqwest::get(google_url)
/// ```
/// > asks Google: "Is this token real?"
/// ## Step 3: Google responds
/// ```json
/// { "email": "...", "name": "...", "picture": "..." }
/// ```
/// ## Step 4: Your backend
/// ```rust
/// check if user exists
/// // YES → use existing user_id
/// // NO → create new user
/// ```
/// ## Step 5: generate JWT
/// ```rust
/// generate_jwt(user_id, email, secret)
/// ```
/// ## Step 6: send to client
/// ```json { "token": "...", "user": {...} } ```
///
/// ## After this?
/// ```text Authorization: Bearer <token> ```
///   * Client stores token and uses:
///   * for all future requests
/// ---
/// # FUNCTION INTERNALS:
///
/// ATTRIBUTES:
/// axum::Json(payload): axum::Json<GoogleAuthRequest> EXPLANATION:
/// - we have struct GoogleAuthRequest and we taking axmu::Json(payload) provided by axum.
/// - thsi deseralize the data intos GoogleAuthRequest struct.
/// - Hence we can used "payload.token"
/// - ⚠️ here the field name must match the struct name => "token"
/// - ou use serde(rename) atrributes.
///
/// Same with State(state):
/// - this defined in /app/app.rs inside
/// - Router::new().route(...).with_state(state)
///
///
/// ### Workflow:
/// - User logs into Google -> Google creates signed ID token -> Frontend receives token -> Frontend sends token to YOUR backend
/// - Your backend asks Google: "is this legit?"
/// - Google validates signature internally ↓ Google responds with user info
///
/// #### Mislenouse:
/// ```rust
/// user.id.ok_or(ApiError::Internal)? // if we dont define 'id!:i64' it becames  Option<T,E>
/// ```
pub async fn google_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(payload): axum::Json<user_stractures::GoogleAuthRequest>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // tooken change every request Hence we use format! to make url dynamic
    let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", payload.token);

    // ✨✨✨
    // request::get() => returns Future + .await => Response
    // then "Response" has the method .json()
    // so we combine 2 tings here. we can seperate Response and body...
    let body: serde_json::Value = reqwest::get(&url)
        .await
        .map_err(|e| {
            tracing::error!("failed to get response from google: {}", e);
            ApiError::Unauthorized
        })?
        .json()
        .await
        .map_err(|err| {
            tracing::error!("Failed to convert user info from google => into .json:{}", err);
            ApiError::Unauthorized
        })?;

    // FIELDS---------------------------
    let email = body["email"].as_str().unwrap_or("");
    let name = body["name"].as_str().unwrap_or("");
    let picture = body["picture"].as_str().unwrap_or("");
    // ---------------------------------

    // IF exciting user.
    let exsisting_user = sqlx::query_as::<_, user_stractures::ExcitingUser>(
        r#"
        SELECT user_id as id
        FROM core_users
        WHERE user_email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(&state.db)
    .await;

    let user_id: i64 = if let Some(user) = exsisting_user.unwrap() {
        user.id
    } else {
        let now = chrono::Utc::now().to_rfc3339();
        // 💔  R: query! is micro, and we cannot assing the type explicetly to micros easily...
        let res = sqlx::query!(
            r#"
            INSERT INTO core_users(
                user_email,user_first_name,
                user_picture,user_login_method,
                user_created_at)
            VALUES(?,?,?,'google',?)
            "#,
            email,
            name,
            picture,
            now
        )
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query:{}", e);
            ApiError::Internal
        })?;

        res.last_insert_rowid()
    };

    // CREATE TOKEN... TY:
    let token = utils::jwt::generate_jwt(user_id, email.to_string(), &state.config.jwt_secret);

    Ok(axum::Json(serde_json::json!({
        "token": token,
        "user":{
            "email":email,
            "name":name,
            "picture": picture,
        }
    })))
}
