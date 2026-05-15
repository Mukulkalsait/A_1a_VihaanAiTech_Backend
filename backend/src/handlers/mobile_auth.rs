// FILE:  /src/handlers/mobile_auth.rs
// Single unified `core_users` table for both Google and Mobile auth.

use axum::{self, Json, extract::State};
use serde;
use serde_json;

use crate::app::AppState;
use crate::errors::ApiError;
use crate::utils::jwt::generate_jwt;

// ---- Register: name + mobile ----

#[derive(serde::Deserialize)]
pub struct MobileRegisterRequest {
    pub name: String,
    pub mobile: String,
}

pub async fn mobile_register(
    State(state): State<AppState>,
    Json(payload): Json<MobileRegisterRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate inputs
    if payload.name.trim().is_empty() || payload.mobile.trim().is_empty() {
        return Err(ApiError::BadRequest("name and mobile are required".to_string()));
    }

    // Check if mobile already exists in core_users
    let existing = sqlx::query_scalar::<_, i64>("SELECT user_id FROM core_users WHERE user_mobile = ?")
        .bind(&payload.mobile)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Existing user query failed: {}", e);
            ApiError::Internal
        })?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("mobile number already registered".to_string()));
    }

    // Insert new user into core_users
    let now = chrono::Utc::now().to_rfc3339();
    let res = sqlx::query(
        r#"INSERT INTO core_users (user_first_name, user_mobile, user_login_method, user_created_at)
           VALUES (?, ?, 'mobile', ?)"#,
    )
    .bind(&payload.name)
    .bind(&payload.mobile)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert user: {}", e);
        ApiError::Internal
    })?;

    let user_id = res.last_insert_rowid();

    // Generate JWT
    let token = generate_jwt(user_id, payload.mobile.clone(), &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user_id,
            "name": payload.name,
            "mobile": payload.mobile,
        }
    })))
}

// ---- Login: just mobile number ----

#[derive(serde::Deserialize)]
pub struct MobileLoginRequest {
    pub mobile: String,
}

pub async fn mobile_login(
    State(state): State<AppState>,
    Json(payload): Json<MobileLoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.mobile.trim().is_empty() {
        return Err(ApiError::BadRequest("mobile is required".to_string()));
    }

    // Find user by mobile in core_users
    let row = sqlx::query_as::<_, MobileUser>(
        "SELECT user_id as id, user_first_name as name, user_mobile as mobile FROM core_users WHERE user_mobile = ?",
    )
    .bind(&payload.mobile)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get the user: {}", e);
        ApiError::Internal
    })?;

    let user = row.ok_or(ApiError::BadRequest("mobile number not registered".to_string()))?;

    // Generate JWT
    let token = generate_jwt(user.id, user.mobile.clone(), &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user.id,
            "name": user.name,
            "mobile": user.mobile,
        }
    })))
}

#[derive(sqlx::FromRow)]
struct MobileUser {
    id: i64,
    name: String,
    mobile: String,
}
