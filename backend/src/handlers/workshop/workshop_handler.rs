// FILE:/src/handlers/workshop/workshop_handler.rs
use axum::{
    Json,
    extract::{Path as AxumPath, State},
};

use crate::{app::AppState, errors::ApiError, modals::workshop::*};

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

pub async fn get_all_workshops(State(state): State<AppState>) -> Result<Json<WorkshopListResponse>, ApiError> {
    let workshops = sqlx::query_as::<_, Workshop>(r#"SELECT * FROM workshops ORDER BY date ASC"#).fetch_all(&state.db).await.map_err(|e| {
        tracing::error!("Failed to fetch workshops: {}", e);
        ApiError::Internal
    })?;

    Ok(Json(WorkshopListResponse { success: true, number: workshops.len() as u32, workshops }))
}
