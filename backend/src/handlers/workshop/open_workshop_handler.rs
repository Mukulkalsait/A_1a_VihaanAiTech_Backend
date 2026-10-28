// FILE:/src/handlers/workshop/open_workshop_handler.rs
use std::path::Path;

use axum::{
    Json,
    extract::{Multipart, State},
};
use sqlx::Row;

use crate::{
    app::AppState,
    errors::ApiError,
    modals::{workshop::*, workshop_registration::*},
    utils::file_handling::file_upload::{save_screenshot, send_confirmation_email},
};
/// ### ───Register Workshop:────────────────────────────────
/// - Api:
/// - Format:-
///```json
///{
/// success:true,
/// message:String,
/// registration_id:i32
///}
///```
pub async fn register_open_workshop(State(state): State<AppState>, mut multipart: Multipart) -> Result<Json<WorkshopRegisterResponse>, ApiError> {
    // 1. Multipart Parcing
    let mut full_name = String::new();
    let mut email = String::new();
    let mut phone_number = String::new();
    let mut country_code = String::from("+91");
    let mut workshop_slug: Option<String> = None;
    let mut screenshot_bytes: Option<axum::body::Bytes> = None;
    let mut screenshot_filename = String::new();
    let mut content_type = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError::BadRequest(format!("Failed to read from data:{}", e)))? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "full_name" => full_name = field.text().await.map_err(|e| ApiError::BadRequest(format!("Failed to read full_name: {}", e)))?,
            "email" => email = field.text().await.map_err(|e| ApiError::BadRequest(format!("Failed to read email: {}", e)))?,
            "phone_number" => phone_number = field.text().await.map_err(|e| ApiError::BadRequest(format!("Failed to read phone_number: {}", e)))?,
            "country_code" => country_code = field.text().await.unwrap_or_else(|_| "+91".to_string()),
            "workshop_slug" => workshop_slug = Some(field.text().await.map_err(|e| ApiError::BadRequest(format!("Failed to read workshop_slug: {}", e)))?),
            "screenshot" => {
                screenshot_filename = field.file_name().unwrap_or("screenshot").to_string();
                content_type = field.content_type().unwrap_or("image/jpeg/jpg").to_string();
                screenshot_bytes = Some(field.bytes().await.map_err(|e| ApiError::BadRequest(format!("Faild to read Screenshot:{}", e)))?);
            },
            _ => {},
        }
    }

    // 2. Validations.
    if full_name.is_empty() || email.is_empty() || phone_number.is_empty() {
        return Err(ApiError::BadRequest("Missing Require Fields.".to_string()));
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(ApiError::BadRequest("Invalid Email.".to_string()));
    }
    let slug = workshop_slug.ok_or_else(|| ApiError::BadRequest("Slug is requried".to_string()))?;
    let bytes = screenshot_bytes.ok_or_else(|| ApiError::BadRequest("Payment Screenshot is Requried.".to_string()))?;

    // 3. Get Workshop from db
    let workshop = sqlx::query_as::<_, Workshop>(r#"SELECT id, slug, name, description, date, amount, currency, capacity, status, created_at, updated_at FROM workshops Where slug = ?"#)
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Faild to fetch workshop by Slug: {} from database:{}", slug, e);
            ApiError::Internal
        })?
        .ok_or_else(|| ApiError::BadRequest(format!("Workshop not found-Slug:{}", slug)))?;

    // 4. Status and capacity check. IMP: OPEN REG
    if workshop.status != "open" {
        return Err(ApiError::BadRequest(format!("Workshop {} is not open to registration [Status:{}]", slug, workshop.status)));
    }
    if let Some(cap) = workshop.capacity {
        let current: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workshop_registrations WHERE workshop_id = ?").bind(workshop.id).fetch_one(&state.db).await.map_err(|e| {
            tracing::error!("Faild to get the 'workshop_registrations' COUNT(*):{}", e);
            ApiError::Internal
        })?;
        if current >= cap as i64 {
            return Err(ApiError::BadRequest(format!("Workshop {} if full ({}/{})", slug, current, cap)));
        }
    }

    // 5.save screenshot
    let upload_dir = Path::new(&state.config.file_upload_path);
    let saved_file = save_screenshot(bytes, &screenshot_filename, &content_type, upload_dir).await?;

    // 6. Insert Registrations.
    let result = sqlx::query(
        r#" INSERT INTO workshop_registrations(workshop_id, full_name, email, phone_number, country_code, payment_screenshot_path, workshop_name, workshop_date, amount) 
            VALUES(?,?,?,?,?,?,?,?,?)
            RETURNING registration_id "#,
    )
    .bind(workshop.id)
    .bind(&full_name)
    .bind(&email)
    .bind(&phone_number)
    .bind(&country_code)
    .bind(&saved_file)
    .bind(&workshop.name)
    .bind(&workshop.date)
    .bind(workshop.amount)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Faild to insert into 'workshop_registrations': {}", e);
        ApiError::Internal
    })?;

    let registration_id: i32 = result.try_get("registration_id").map_err(|e| {
        tracing::error!("Faild to get registration_id:{}", e);
        ApiError::Internal
    })?;

    let name_clone = full_name.clone();
    let email_clone = email.clone();
    let workshop_name_clone = workshop.name.clone();
    tokio::spawn(async move {
        let _ = send_confirmation_email(&name_clone, &email_clone, registration_id, &workshop_name_clone).await;
    });

    Ok(Json(WorkshopRegisterResponse { success: true, message: "Registration Successful!".to_string(), registration_id }))
}
