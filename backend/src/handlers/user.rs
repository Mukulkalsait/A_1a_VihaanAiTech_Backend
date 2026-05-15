// FILE: ./src/handlers/user.rs

use argon2::{
    Argon2, Error,
    password_hash::{self, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{self, Json, extract::State};
use serde;
use sqlx::query;
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
    pub mobile: Option<String>,
    pub dob: Option<String>,
}

/// # Hashing Contains 2 tings,
/// 1. Salt (Random Data Befroe Hashing)
///  - SaltString::generate -> generate new saltstring
///  - ::rand_core::OsRng -> rand core -> OS level Random Generator.
///
///---
/// 2. Orighal Password.
///  - Argon2.default().hash_password() => default password hashing Algo.
///  - passwd.as_bytes => Hashing Algo work on RAW BITES.
///  - map_err already convert the hashed_password into Result<T,E> ⭐⭐⭐⭐
///  - map_erro contains the error tyep soe return type dont need one.
///
fn password_hasher(passwd: String) -> Result<String, ApiError> {
    let created_salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let hashed_password = Argon2::default()
        .hash_password(passwd.as_bytes(), &created_salt)
        .map_err(|e| {
            tracing::error!("Failed to Hashihg: {}", e);
            ApiError::Internal
        })?
        .to_string();
    Ok(hashed_password)
}

/// # Creating User
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<serde_json::Value>, errors::ApiError> {
    let now = chrono::Utc::now().to_rfc3339(); // Time
    // let new_id = uuid::Uuid::new_v4(); // impliment latger
    let hashed_password = password_hasher(payload.password)?;

    sqlx::query(
        r#"
        INSERT INTO core_users (
            user_email,
            user_first_name,
            user_last_name,
            user_password,
            user_mobile,
            user_dob,
            user_created_at
        )
        VALUES (?,?,?, ?,?,?, ?)
        "#,
    )
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&hashed_password)
    .bind(&payload.mobile)
    .bind(&payload.dob)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to run db Query : {}", e);
        ApiError::Internal
    })?;

    Ok(Json(serde_json::json!({"status":"user created"})))
}

// ---------------------------------------------------------------------------------
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub email: String,
}

pub async fn list_user(State(state): State<AppState>) -> Result<Json<Vec<User>>, errors::ApiError> {
    let users = sqlx::query_as::<_, User>(
        r#"
            SELECT
                user_id as "id!: i64",
                user_first_name as first_name,
                user_email as email
            FROM core_users
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to reun db query: {}", e);
        ApiError::Internal
    })?;

    Ok(Json(users))
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

// IMP:
// id TEXT PRIMARY KEY,
// Y:
// user_first_name TEXT NOT NULL,
// user_last_name TEXT NOT NULL,
// user_mobile TEXT ,
// user_dob TEXT,
// user_email TEXT,
// B:
// user_verified INTEGER NOT NULL DEFAULT 0,
// user_password TEXT NOT NULL,
// R:
// user_verification_token TEXT,
// user_token_expires_at TEXT,
// G:
// user_role TEXT NOT NULL DEFAULT 'user' CHECK (user_role IN ('admin', 'user')),
// user_created_at TEXT DEFAULT CURRENT_TIMESTAMP,
// user_updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
