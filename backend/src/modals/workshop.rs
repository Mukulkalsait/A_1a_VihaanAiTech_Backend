use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub type WorkshopID = uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Workshop {
    pub id: WorkshopID,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub date: String,

    pub amount: f64,
    pub currency: String,
    pub capacity: Option<i32>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,

    pub time_label: Option<String>,
    pub timezone: Option<String>,
    pub whatsapp_group_url: Option<String>,
    pub whatsapp_qr_image_path: Option<String>,
    pub payment_qr_image_path: Option<String>,

    pub support_phone: Option<String>,
    pub support_whatsapp_url: Option<String>,
    pub terms_url: Option<String>,
    pub landing_tagline: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct WorkshopBadge {
    pub icon: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct WorkshopDetailsResponse {
    pub success: bool,
    pub workshop: Workshop,
    pub badges: Vec<WorkshopBadge>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct WorkshopListResponse {
    pub success: bool,
    pub number: u32,
    pub workshops: Vec<Workshop>,
}
