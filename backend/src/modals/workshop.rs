use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

pub type WorkshopID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum WorkshopStatus {
    Draft,
    Open,
    Closed,
    Cancelled,
    Archived,
}

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
    pub status: WorkshopStatus,
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

#[derive(Debug, Serialize)]
pub struct WorkshopCreationResponse {
    pub success: bool,
    pub workshop: Workshop,
}
