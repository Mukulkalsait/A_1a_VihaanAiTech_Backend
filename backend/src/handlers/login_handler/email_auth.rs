// FILE:  /src/handlers/email_auth.rs

use axum::{self, Json, extract::State};
use serde_json;

use crate::{
    app::AppState,
    errors::ApiError,
    modals::registration_modals::{EmailLoginRequest, EmailRegisterRequest, EmailUser},
    utils::json_web_token::generate_jwt,
};

pub async fn email_register(State(state): State<AppState>, Json(payload): Json<EmailRegisterRequest>) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.name.trim().is_empty() || payload.email.trim().is_empty() {
        return Err(ApiError::BadRequest("Name and Email are Required.".to_string()));
    }

    let existing = sqlx::query_scalar::<_, i64>("SELECT user_id FROM core_users WHERE user_email = ?").bind(&payload.email).fetch_optional(&state.db).await.map_err(|e| {
        tracing::error!("Existing User query faild: {}", e);
        ApiError::Internal
    })?;
    if existing.is_some() {
        return Err(ApiError::BadRequest("Email already registered".to_string()));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let user_id = uuid::Uuid::new_v4();
    let _res = sqlx::query(r#"INSERT INTO core_users (user_id, user_first_name, user_email,user_login_method, user_created_at) VALUES(?,?,?,'email',?)"#)
        .bind(user_id)
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&now)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Faild to insert user: {}", e);
            ApiError::Internal
        })?;

    let token = generate_jwt(user_id, payload.email.clone(), &state.config.jwt_secret)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user": {
            "id": user_id,
            "name": payload.name,
            "email": payload.email,
        }
    })))
}

pub async fn email_login(State(state): State<AppState>, Json(payload): Json<EmailLoginRequest>) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.email.trim().is_empty() {
        return Err(ApiError::BadRequest("Email is requreid".to_string()));
    }

    let row = sqlx::query_as::<_, EmailUser>("SELECT user_id as id, user_first_name as name, user_email as email FROM core_users WHERE user_email = ? ")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Faild to get the user:{}", e);
            ApiError::Internal
        })?;

    let user = row.ok_or(ApiError::BadRequest("Email is not regustered".to_string()))?;
    let token = generate_jwt(user.id, user.email.clone(), &state.config.jwt_secret);

    Ok(Json(serde_json::json!({
    "token":token,
        "user":{
            "id":user.id,
            "name":user.name,
            "email":user.email,
        }
    })))
}
