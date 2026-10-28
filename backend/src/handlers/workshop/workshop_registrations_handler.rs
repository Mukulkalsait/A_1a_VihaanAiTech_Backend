// FILE:/src/handlers/workshop/workshop_handler.rs

use axum::{
    Json,
    extract::{Path as AxumPath, State},
};
use sqlx::Row;

use crate::{app::AppState, errors::ApiError, modals::workshop_registration::*};

/// ### ───Get All Registrations:────────────────────────────────
/// - Api:
/// - Format:-
///```json
///{
///  success:bool,
///  count:usize,
///  registrations: Vec<WorkshopRegistration>
///}
///```
pub async fn list_all_registrations(State(state): State<AppState>) -> Result<Json<RegistrationListResponse>, ApiError> {
    let registrations = sqlx::query_as::<_, WorkshopRegistration>(
        r#" SELECT 
                registration_id, user_id, workshop_id, workshop_name, workshop_date, amount,
                payment_screenshot_path, payment_status, registration_status,
                attended, attended_at, created_at, updated_at,
                full_name, email, phone_number, country_code
            FROM workshop_registrations
            ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Faild to fetch all the workshop_registrations :{}", e);
        ApiError::Internal
    })?;

    Ok(Json(RegistrationListResponse { success: true, count: registrations.len(), registrations }))
}

/// ### ───Get Single Registration Data:────────────────────────────────
/// - Api:
/// - Format:-
///```json
///{
///  success:bool,
///  registration,
///}
///```
pub async fn get_registration(State(state): State<AppState>, AxumPath(id): AxumPath<i32>) -> Result<Json<SingleRegistrationResponse>, ApiError> {
    let registration = sqlx::query_as(
        r#" SELECT
                registration_id, user_id, workshop_id, workshop_name, workshop_date, amount,
                payment_screenshot_path, payment_status, registration_status,
                attended, attended_at, created_at, updated_at,
                full_name, email, phone_number, country_code
            FROM workshop_registrations 
            WHERE registration_id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Faild to fetch registrations {id}:{}", e);
        ApiError::Internal
    })?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(SingleRegistrationResponse { success: true, registration }))
}

/// ### ───Verify Registration:────────────────────────────────
/// - Api:
/// - Format:-
///```json
///{
///  success:bool,
///  message:String,
///}
///```
pub async fn verify_registration(State(state): State<AppState>, AxumPath(id): AxumPath<i32>) -> Result<Json<RegistrationActionResponse>, ApiError> {
    let res = sqlx::query(r#" UPDATE workshop_registrations SET payment_status = 'verified', updated_at = CURRENT_TIMESTAMP WHERE registration_id = ? AND payment_status != 'verified' "#)
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Faild to verify registration: {id}:{}", e);
            ApiError::Internal
        })?;
    if res.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(Json(RegistrationActionResponse { success: true, message: format!("Registration id:{}, Verification Success.", id) }))
}

/// ### ───Delete Registration:────────────────────────────────
/// - Api:
/// - Format:-
///```json
///{
///  success:bool,
///  message:String,
///}
///```
pub async fn delete_registration(State(state): State<AppState>, AxumPath(id): AxumPath<i32>) -> Result<Json<RegistrationActionResponse>, ApiError> {
    let row = sqlx::query(r#"SELECT payment_screenshot_path FROM workshop_registrations WHERE registration_id = ?"#).bind(id).fetch_optional(&state.db).await.map_err(|e| {
        tracing::error!("Faild to fetch registration data: {}", e);
        ApiError::Internal
    })?;

    let Some(row) = row else {
        return Err(ApiError::NotFound);
    };
    let screenshot: Option<String> = row.try_get("payment_screenshot_path").map_err(|e| {
        tracing::error!("Faild to read 'payment_screenshot_path':{}", e);
        ApiError::Internal
    })?;
    if let Some(path) = screenshot {
        let full_path = std::path::Path::new(&state.config.file_upload_path).join(&path);
        if let Err(e) = tokio::fs::remove_file(&full_path).await {
            tracing::warn!("Failed to remove screenshot {:?}:{}", full_path, e);
        }
    }

    let res = sqlx::query(r#"DELETE FROM workshop_registrations WHERE registration_id = ?"#).bind(id).execute(&state.db).await.map_err(|e| {
        tracing::error!("Faild to delete registration. ID:{}, Err:{}", id, e);
        ApiError::Internal
    })?;
    if res.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(RegistrationActionResponse { success: true, message: format!("Registration id:{} Successfully DELETED", id) }))
}
