// FILE: ./src/handlers/google_auth.rs

// use anyhow;
use axum;
use reqwest;
use serde_json;

// use std::fmt::format;
// EXT:
use crate::app::AppState;
use crate::{errors::ApiError, modals::auth_modal::*, utils};

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
pub async fn google_auth(axum::extract::State(state): axum::extract::State<AppState>, axum::Json(payload): axum::Json<GoogleAuthRequest>) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // tooken change every request Hence we use format! to make url dynamic
    // let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", payload.token);

    // ✨✨✨
    // request::get() => returns Future + .await => Response
    // then "Response" has the method .json()
    // so we combine 2 tings here. we can seperate Response and body...
    // let body: serde_json::Value = reqwest::get(&url).await.map_err(|e| { tracing::error!("failed to get response from google: {}", e); ApiError::Unauthorized })?.json().await.map_err(|err| { tracing::error!("Failed to convert user info from google => into .json:{}", err); ApiError::Unauthorized })?; // FIELDS---------------------------

    let client = reqwest::Client::new();
    let url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let response = client.get(url).bearer_auth(&payload.token).send().await.map_err(|e| {
        tracing::error!("Falied to connect to Google: {}", e);
        ApiError::Unauthorized
    })?;

    if !response.status().is_success() {
        tracing::error!("Google Token Regected.");
        return Err(ApiError::Unauthorized);
    }

    let body: serde_json::Value = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse google user info at google_auth: {}", e);
        ApiError::Unauthorized
    })?;

    tracing::debug!("Googel user info: {:?}", body);

    let email = body["email"].as_str().unwrap_or("");
    let name = body["name"].as_str().unwrap_or("");
    let picture = body["picture"].as_str().unwrap_or("");
    let exsisting_user = sqlx::query_as::<_, ExcitingUserID>(r#" SELECT user_id as id FROM core_users WHERE user_email = ? "#).bind(email).fetch_optional(&state.db).await;

    let user_id: uuid::Uuid = if let Some(user) = exsisting_user.unwrap() {
        user.id
    } else {
        // 💔  R: query! is micro, and we cannot assing the type explicetly to micros easily...
        let now = chrono::Utc::now().to_rfc3339();
        let user_id = uuid::Uuid::new_v4();
        let _res = sqlx::query(r#" INSERT INTO core_users(user_id, user_email, user_first_name, user_picture, user_login_method, user_created_at) VALUES (?,?, ?, ?, 'google', ?) "#)
            .bind(user_id)
            .bind(email)
            .bind(name)
            .bind(picture)
            .bind(now)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {}", e);
                ApiError::Internal
            })?;
        user_id
    };

    let token = utils::json_web_token::generate_jwt(user_id, email.to_string(), &state.config.jwt_secret)?;
    Ok(axum::Json(serde_json::json!({
        "token": token,
        "user":{
            "email":email,
            "name":name,
            "picture": picture,
        }
    })))
}
