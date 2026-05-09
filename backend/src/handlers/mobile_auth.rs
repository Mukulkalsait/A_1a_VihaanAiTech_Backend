// FILE:  /src/handlers/mobile_auth.re

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

    // Check if mobile already exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM mobile_users WHERE mobile = ?")
        .bind(&payload.mobile)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("mobile number already registered".to_string()));
    }

    // Insert new mobile user
    let now = chrono::Utc::now().to_rfc3339();
    let res = sqlx::query("INSERT INTO mobile_users (name, mobile, created_at) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.mobile)
        .bind(&now)
        .execute(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let user_id = res.last_insert_rowid();

    // Generate JWT
    let token = generate_jwt(user_id, payload.mobile.clone(), &state.config.jwt_secret);

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

    // Find user by mobile
    let row = sqlx::query_as::<_, MobileUser>("SELECT id, name, mobile FROM mobile_users WHERE mobile = ?")
        .bind(&payload.mobile)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let user = row.ok_or(ApiError::BadRequest("mobile number not registered".to_string()))?;

    // Generate JWT
    let token = generate_jwt(user.id, user.mobile.clone(), &state.config.jwt_secret);

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
