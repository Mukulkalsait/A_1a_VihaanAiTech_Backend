// FILE: ./src/handlers/user.rs
use argon2::{Argon2, password_hash::PasswordHasher};
use axum::{self, Json, extract::State};

use crate::{
    app::AppState,
    errors::{self, ApiError},
    modals::users_mod::{CreateUser, User},
    utils,
};

// ---------------------------------------------------------------------------------

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
pub async fn create_user(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Result<Json<serde_json::Value>, errors::ApiError> {
    let now = utils::helper::time_now_utc_string(); // Time
    let user_id = uuid::Uuid::new_v4(); // impliment latger
    let hashed_password = password_hasher(payload.password)?;

    sqlx::query(
        r#" INSERT INTO core_users ( user_id, user_email, user_first_name, user_last_name, user_password, user_mobile, user_dob, user_created_at)
        VALUES (?,?,?,?,?,?,?, ?)
        "#,
    )
    .bind(user_id)
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

pub async fn list_user(State(state): State<AppState>) -> Result<Json<Vec<User>>, errors::ApiError> {
    let users = sqlx::query_as::<_, User>(r#" SELECT user_id as id, user_first_name as first_name, user_email as email FROM core_users "#).fetch_all(&state.db).await.map_err(|e| {
        tracing::error!("Failed to reun db query: {}", e);
        ApiError::Internal
    })?;

    Ok(Json(users))
}
