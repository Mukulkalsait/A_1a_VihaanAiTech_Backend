use anyhow;
use axum;
use reqwest;
use serde;
use serde_json;
use std::fmt::format;
// EXT:

use crate::app::AppState;
use crate::errors::ApiError;
use crate::utils::jwt::generate_jwt;

#[derive(serde::Deserialize)]
pub struct GoogleAuthRequest {
    pub token: String,
}

pub async fn google_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(payload): axum::Json<GoogleAuthRequest>,
    // token {"token":"google_id_token"}
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // resp(response)
    // reqwest::get(&urs) -> asking google if this token ok?
    // otherwise Unauthorised
    let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", payload.token);
    let resp = reqwest::get(&url).await.map_err(|_| ApiError::Unauthorized)?;
    // run the http request and convert the response into JSON... Y:
    let body: serde_json::Value = resp.json().await.map_err(|_| ApiError::Unauthorized)?;

    // FIELDS---------------------------
    let email = body["email"].as_str().unwrap_or("");
    let name = body["name"].as_str().unwrap_or("");
    let picture = body["picture"].as_str().unwrap_or("");
    // ---------------------------------

    // IF exciting user.
    let exsisting_user = sqlx::query!("SELECT id as 'id!:i64' FROM core_users WHERE email = ?", email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let user_id: i64 = if let Some(user) = exsisting_user {
        // user.id.ok_or(ApiError::Internal)? if we dont define 'id!:i64' it becames  Option<T,E>
        // then this used or we can use direct user.io.unwrap()
        user.id
    } else {
        let now = chrono::Utc::now().to_rfc3339();
        let res = sqlx::query!(
            r#"INSERT INTO core_users (email,first_name,picture,login_method,created_at) VALUES(?,?,?,'google',?)"#,
            email,
            name,
            picture,
            now
        )
        .execute(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

        res.last_insert_rowid()
    };

    // CREATE TOKEN...
    let token = generate_jwt(user_id, email.to_string(), &state.config.jwt_secret);

    Ok(axum::Json(serde_json::json!({
        "token": token,
        "user":{
            "email":email,
            "name":name,
            "picture": picture,
        }
    })))
}
