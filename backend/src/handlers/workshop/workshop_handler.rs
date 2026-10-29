// FILE:/src/handlers/workshop/workshop_handler.rs
use axum::{
    Json,
    extract::{Multipart, Path as AxumPath, State},
};

use crate::{
    app::AppState,
    errors::ApiError,
    handlers::admin::AdminUser,
    modals::workshop::*,
    utils::{self, helper::*},
};

pub async fn get_all_workshops(State(state): State<AppState>) -> Result<Json<WorkshopListResponse>, ApiError> {
    let workshops = sqlx::query_as::<_, Workshop>(r#"SELECT * FROM workshops ORDER BY date ASC"#).fetch_all(&state.db).await.map_err(|e| {
        tracing::error!("Failed to fetch workshops: {}", e);
        ApiError::Internal
    })?;

    Ok(Json(WorkshopListResponse { success: true, number: workshops.len() as u32, workshops }))
}

pub async fn get_workshop_by_slug(State(state): State<AppState>, AxumPath(slug): AxumPath<String>) -> Result<Json<WorkshopDetailsResponse>, ApiError> {
    let workshop = sqlx::query_as::<_, Workshop>(r#" SELECT * FROM workshops WHERE slug = ? "#)
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("failed to fetch workshop with slug:{} from db.'workshops'", e);
            ApiError::Internal
        })?
        .ok_or(ApiError::NotFound)?;
    let badges = sqlx::query_as::<_, WorkshopBadge>(r#"SELECT icon, text FROM workshop_badges WHERE workshop_id = ? ORDER BY display_order"#).bind(workshop.id).fetch_all(&state.db).await.map_err(|e| {
        tracing::error!("Faield to fetch data from 'workshop_badges' table:{}", e);
        ApiError::Internal
    })?;

    Ok(Json(WorkshopDetailsResponse { success: true, workshop, badges }))
}

pub async fn new_workshop(State(state): State<AppState>, _admin: AdminUser, mut multipart: Multipart) -> Result<Json<WorkshopCreationResponse>, ApiError> {
    // id: WorkshopID,
    // created_at: String,
    // updated_at: String,
    let mut slug = String::new();
    let mut name = String::new();
    let mut description: Option<String> = None;
    let mut date = String::new();
    let mut amount: Option<f64> = None;
    let mut currency = String::from("INR");
    let mut capacity: Option<i32> = None;
    let mut status = WorkshopStatus::Open;

    let mut time_label: Option<String> = None;
    let mut timezone: Option<String> = None;
    let mut whatsapp_group_url: Option<String> = None;
    let mut whatsapp_qr_image_path: Option<String> = None;
    let mut payment_qr_image_path: Option<String> = None;
    let mut support_phone: Option<String> = None;
    let mut support_whatsapp_url: Option<String> = None;
    let mut terms_url: Option<String> = None;
    let mut landing_tagline: Option<String> = None;

    while let Some(f) = multipart.next_field().await.map_err(|e| ApiError::BadRequest(format!("Faild to get next field:{}", e)))? {
        let field_name = f.name().unwrap_or("").to_string(); // Y: 

        match field_name.as_str() {
            "slug" => slug = f.text().await.map_err(|e| ApiError::BadRequest(format!("Slug extraction failed:{}", e)))?,
            "name" => name = f.text().await.map_err(|e| ApiError::BadRequest(format!("Name extraction failed:{}", e)))?,
            "description" => description = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("Description extraction failed:{}", e)))?),
            "date" => date = f.text().await.map_err(|e| ApiError::BadRequest(format!("Date extraction failed:{}", e)))?,
            "amount" => {
                let a = f.text().await.map_err(|e| ApiError::BadRequest(format!("Date extraction failed:{}", e)))?;
                amount = Some(a.parse::<f64>().map_err(|e| ApiError::BadRequest(format!("Invalid Amount Format:{}", e)))?)
            },
            "currency" => currency = f.text().await.map_err(|e| ApiError::BadRequest(format!("Currency extraction failed:{}", e)))?,
            "capacity" =>
                capacity = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("Capacity extraction failed:{}", e)))?.parse::<i32>().map_err(|e| ApiError::BadRequest(format!("Parcing failed:{}", e)))?),
            "status" => status = Workshop::parse_status(f.text().await.map_err(|e| ApiError::BadRequest(format!("Status extraction failed:{}", e)))?.as_str())?,
            "time_label" => time_label = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("time_label extraction failed:{}", e)))?),
            "timezone" => timezone = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("timezone extraction failed:{}", e)))?),
            "whatsapp_group_url" => whatsapp_group_url = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("whatsapp_group_url extraction failed:{}", e)))?),
            "whatsapp_qr_image_path" => whatsapp_qr_image_path = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("whatsapp_qr_image_path extraction failed:{}", e)))?),
            "payment_qr_image_path" => payment_qr_image_path = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("payment_qr_image_path extraction failed:{}", e)))?),
            "support_phone" => support_phone = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("support_phone extraction failed:{}", e)))?),
            "support_whatsapp_url" => support_whatsapp_url = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("support_whatsapp_url extraction failed:{}", e)))?),
            "terms_url" => terms_url = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("terms_url extraction failed:{}", e)))?),
            "landing_tagline" => landing_tagline = Some(f.text().await.map_err(|e| ApiError::BadRequest(format!("landing_tagline extraction failed:{}", e)))?),
            _ => {},
        }
    }
    if slug.trim().is_empty() || name.trim().is_empty() || date.trim().is_empty() {
        return Err(ApiError::BadRequest("Please provide all the requred details".to_string()));
    }
    let amount = amount.ok_or_else(|| ApiError::BadRequest("amount is required".to_string()))?;
    if amount <= 0.0 {
        return Err(ApiError::BadRequest(format!("Valid Amount is required:{}", amount)));
    }
    if !is_iso_date(&date) {
        return Err(ApiError::BadRequest(format!("date must be YYYY-MM-DD format not: {}", date)));
    }
    if capacity.is_some_and(|cap| cap <= 0) {
        return Err(ApiError::BadRequest("Capacity must be possitive".to_string()));
    }

    let exists: Option<(String,)> = sqlx::query_as(r#"SELECT id FROM workshops WHERE slug = ?"#).bind(&slug).fetch_optional(&state.db).await.map_err(|e| {
        tracing::error!("DB Query Fail (to GET all Slugs):{}", e);
        ApiError::Internal
    })?;
    if exists.is_some() {
        return Err(ApiError::BadRequest(format!("a Workshop with slug:{}, already exists", slug)));
    }

    let id = uuid::Uuid::new_v4();
    let now = utils::helper::time_now_utc_string();

    sqlx::query(
        r#"
    INSERT INTO workshops(
        id, slug, name, description, date, amount, currency, capacity, status, created_at, updated_at,
        time_label, timezone, whatsapp_group_url, whatsapp_qr_image_path, payment_qr_image_path, support_phone,
        support_whatsapp_url, terms_url, landing_tagline
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(id)
    .bind(&slug)
    .bind(&name)
    .bind(&description)
    .bind(&date)
    .bind(amount)
    .bind(&currency)
    .bind(capacity)
    .bind(status)
    .bind(&now)
    .bind(&now)
    .bind(&time_label)
    .bind(&timezone)
    .bind(&whatsapp_group_url)
    .bind(&whatsapp_qr_image_path)
    .bind(&payment_qr_image_path)
    .bind(&support_phone)
    .bind(&support_whatsapp_url)
    .bind(&terms_url)
    .bind(&landing_tagline)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("faild to insert new workshop:{}", e);
        ApiError::Internal
    })?;

    let created = sqlx::query_as::<_, Workshop>(r#"SELECT * FROM workshops where id = ?"#).bind(id).fetch_one(&state.db).await.map_err(|e| {
        tracing::error!("Failed to get the latest created workshop information:{}", e);
        ApiError::Internal
    })?;

    Ok(Json(WorkshopCreationResponse { success: true, workshop: created }))
}
