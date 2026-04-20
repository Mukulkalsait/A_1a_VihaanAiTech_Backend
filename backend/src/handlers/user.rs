use axum::{self, Json, extract::State};
use serde;
//EXT
use crate::app::AppState;
use crate::errors::{self, ApiError};
//INT

// ---------------------------------------------------------------------------------
#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, errors::ApiError> {
    let now = chrono::Utc::now().to_rfc3339(); // Time

    sqlx::query!(
        r#"INSERT INTO core_users (email,first_name,last_name,password_hash,created_at) VALUES(?,?,?,?,?)"#,
        payload.email,
        payload.first_name,
        payload.last_name,
        payload.password,
        now
    )
    .execute(&state.db)
    .await
    .map_err(|_| errors::ApiError::Internal)?;

    Ok(Json(serde_json::json!({"status":"user created"})))
}

// ---------------------------------------------------------------------------------
#[derive(serde::Serialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub email: String,
}

pub async fn list_user(State(state): State<AppState>) -> Result<Json<serde_json::Value>, errors::ApiError> {
    let users = sqlx::query_as!(User, "SELECT id, email, first_name FROM core_users")
        .fetch_all(&state.db)
        .await
        .map_err(|_| ApiError::Internal)?;
    Ok(Json(serde_json::json!(users)))
}

// ---------------------------------------------------------------------------------
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub picture: Option<String>,
    pub membership: String,
}
