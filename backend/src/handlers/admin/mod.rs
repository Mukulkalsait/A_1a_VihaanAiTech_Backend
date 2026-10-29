use axum::{Json, extract::FromRequestParts, http::header};
use serde_json::{Value, json};
use sqlx::Row;

use crate::{
    app::AppState,
    errors::ApiError,
    handlers::admin,
    modals::{jwt_modals::Claims, users_mod::UserID},
    utils::json_web_token::verify_jwt,
};

pub struct AdminUser {
    pub user_id: UserID,
    pub email: String,
}

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ApiError; // R: REJECT REQ

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok()).ok_or(ApiError::Unauthorized)?;
        let token = auth_header.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;
        let claims = verify_jwt(token, &state.config.jwt_secret)?;
        let row = sqlx::query(r#"SELECT user_role, user_email FROM core_users WHERE user_id = ?"#)
            .bind(claims.sub)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Faild to load User -> Admin check, Faild to get data from core_users: {}", e);
                ApiError::Internal
            })?
            .ok_or(ApiError::Unauthorized)?;
        let role: String = row.try_get("user_role").map_err(|e| {
            tracing::error!("Faild to extract user_role from row:{}", e);
            ApiError::Internal
        })?;
        if role != "admin" {
            return Err(ApiError::Forbidden);
        }

        let email: String = row.try_get("user_email").map_err(|e| {
            tracing::error!("Failed to extract email from row:{}", e);
            ApiError::Internal
        })?;

        Ok(Self { user_id: claims.sub, email })
    }
}

pub async fn whoami(admin: AdminUser) -> Json<Value> {
    Json(json!({
        "success":true,
        "user_id":admin.user_id,
        "email":admin.email,
        "role":"admin",
    }))
}
