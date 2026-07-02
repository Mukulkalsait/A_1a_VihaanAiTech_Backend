use std::path::Path;
use axum::{Json, extract::State};
use axum::extract::Path as AxumPath;
use serde_json::json;
use sqlx::Row;

use crate::app::AppState;
use crate::errors::ApiError;
use crate::utils::file_handling::file_upload::{save_screenshot, send_confirmation_email};

// ─── Register Handler ────────────────────────────────────────
pub async fn register_open_workshop(
    State(state): State<AppState>,
    mut multipart: axum::extract::Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut full_name = String::new();
    let mut email = String::new();
    let mut phone_number = String::new();
    let mut country_code = String::from("+91");
    let mut screenshot_bytes: Option<axum::body::Bytes> = None;
    let mut screenshot_filename = String::new();
    let mut content_type = String::new();
    let mut workshop_name = String::from("Data Science Workshop");
    let mut workshop_date = String::from("2026-06-07");

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::BadRequest(format!("Failed to read form data: {}", e))
    })? {
        let field_name = field.name().unwrap_or("");

        match field_name {
            "full_name" => {
                full_name = field.text().await.map_err(|e| {
                    ApiError::BadRequest(format!("Failed to read full_name: {}", e))
                })?;
            }
            "email" => {
                email = field.text().await.map_err(|e| {
                    ApiError::BadRequest(format!("Failed to read email: {}", e))
                })?;
            }
            "phone_number" => {
                phone_number = field.text().await.map_err(|e| {
                    ApiError::BadRequest(format!("Failed to read phone_number: {}", e))
                })?;
            }
            "country_code" => {
                country_code = field.text().await.unwrap_or_else(|_| "+91".to_string());
            }
            "workshop_name" => {
                workshop_name = field.text().await.unwrap_or_else(|_| "Data Science Workshop".to_string());
            }
            "workshop_date" => {
                workshop_date = field.text().await.unwrap_or_else(|_| "2026-06-07".to_string());
            }
            "screenshot" => {
                screenshot_filename = field.file_name().unwrap_or("screenshot").to_string();
                content_type = field.content_type().unwrap_or("image/jpeg").to_string();
                screenshot_bytes = Some(field.bytes().await.map_err(|e| {
                    ApiError::BadRequest(format!("Failed to read screenshot: {}", e))
                })?);
            }
            _ => {}
        }
    }

    if full_name.is_empty() || email.is_empty() || phone_number.is_empty() {
        return Err(ApiError::BadRequest("Missing required fields".to_string()));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(ApiError::BadRequest("Invalid email address".to_string()));
    }

    let bytes = screenshot_bytes
        .ok_or_else(|| ApiError::BadRequest("Payment screenshot is required".to_string()))?;

    let upload_dir = Path::new(&state.config.file_upload_path);
    let saved_file = save_screenshot(bytes, &screenshot_filename, &content_type, upload_dir).await?;

    let result = sqlx::query(
        r#"
        INSERT INTO open_workshop_registrations (
            full_name, email, phone_number, country_code,
            payment_screenshot_path, workshop_name, workshop_date
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(&full_name)
    .bind(&email)
    .bind(&phone_number)
    .bind(&country_code)
    .bind(&saved_file)
    .bind(&workshop_name)
    .bind(&workshop_date)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert registration: {}", e);
        ApiError::Internal
    })?;

    let registration_id: i32 = result.get("id");

    let name_clone = full_name.clone();
    let email_clone = email.clone();
    tokio::spawn(async move {
        let _ = send_confirmation_email(&name_clone, &email_clone, registration_id).await;
    });

    Ok(Json(json!({
        "success": true,
        "message": "Registration Successful!",
        "registration_id": registration_id
    })))
}

// ─── List All Registrations ────────────────────────────────
pub async fn list_registrations(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let rows = sqlx::query(
        r#"
        SELECT 
            id, full_name, email, phone_number, country_code,
            payment_screenshot_path, workshop_name, workshop_date,
            amount, payment_status, registered_at, updated_at
        FROM open_workshop_registrations
        ORDER BY registered_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch registrations: {}", e);
        ApiError::Internal
    })?;

    let mut registrations = Vec::new();
    for row in rows {
        registrations.push(json!({
            "id": row.get::<i32, _>("id"),
            "full_name": row.get::<String, _>("full_name"),
            "email": row.get::<String, _>("email"),
            "phone_number": row.get::<String, _>("phone_number"),
            "country_code": row.get::<String, _>("country_code"),
            "payment_screenshot_path": row.get::<String, _>("payment_screenshot_path"),
            "workshop_name": row.get::<String, _>("workshop_name"),
            "workshop_date": row.get::<String, _>("workshop_date"),
            "amount": row.get::<f64, _>("amount"),
            "payment_status": row.get::<String, _>("payment_status"),
            "registered_at": row.get::<String, _>("registered_at"),
            "updated_at": row.get::<String, _>("updated_at"),
        }));
    }

    Ok(Json(json!({
        "success": true,
        "count": registrations.len(),
        "registrations": registrations
    })))
}

// ─── Get Single Registration ───────────────────────────────
pub async fn get_registration(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<i32>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let row = sqlx::query(
        r#"
        SELECT 
            id, full_name, email, phone_number, country_code,
            payment_screenshot_path, workshop_name, workshop_date,
            amount, payment_status, registered_at, updated_at
        FROM open_workshop_registrations
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch registration: {}", e);
        ApiError::Internal
    })?;

    let row = row.ok_or(ApiError::NotFound)?;

    Ok(Json(json!({
        "success": true,
        "registration": {
            "id": row.get::<i32, _>("id"),
            "full_name": row.get::<String, _>("full_name"),
            "email": row.get::<String, _>("email"),
            "phone_number": row.get::<String, _>("phone_number"),
            "country_code": row.get::<String, _>("country_code"),
            "payment_screenshot_path": row.get::<String, _>("payment_screenshot_path"),
            "workshop_name": row.get::<String, _>("workshop_name"),
            "workshop_date": row.get::<String, _>("workshop_date"),
            "amount": row.get::<f64, _>("amount"),
            "payment_status": row.get::<String, _>("payment_status"),
            "registered_at": row.get::<String, _>("registered_at"),
            "updated_at": row.get::<String, _>("updated_at"),
        }
    })))
}

// ─── Verify Registration ────────────────────────────────────
pub async fn verify_registration(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<i32>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let result = sqlx::query(
        r#"
        UPDATE open_workshop_registrations
        SET payment_status = 'verified', updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND payment_status != 'verified'
        "#
    )
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to verify registration: {}", e);
        ApiError::Internal
    })?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({
        "success": true,
        "message": format!("Registration {} verified successfully", id)
    })))
}

// ─── Delete Registration ────────────────────────────────────
pub async fn delete_registration(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<i32>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get the screenshot path first
    let row = sqlx::query(
        "SELECT payment_screenshot_path FROM open_workshop_registrations WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch registration: {}", e);
        ApiError::Internal
    })?;

    if let Some(row) = row {
        let screenshot_path: String = row.get("payment_screenshot_path");
        let _ = tokio::fs::remove_file(format!("./uploads/{}", screenshot_path)).await;
    }

    // Delete from database
    let result = sqlx::query(
        "DELETE FROM open_workshop_registrations WHERE id = ?"
    )
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete registration: {}", e);
        ApiError::Internal
    })?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({
        "success": true,
        "message": format!("Registration {} deleted successfully", id)
    })))
}
