use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub enum OpenWorkshopPaymentStatus {
    Completed,
    Pending,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OpenWorkshopRegistration {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub country_code: String,
    pub payment_screenshot_path: String,
    pub workshop_name: String,
    pub workshop_date: String,
    pub amount: f64,
    pub payment_status: OpenWorkshopPaymentStatus,
    pub resigtered_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenWOrkshopRegistrationRequest {
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OpenWorkshopRegistrationResponse {
    pub success: bool,
    pub message: String,
    pub registration_id: Option<i32>,
}

// Add this for list response
#[derive(Debug, Serialize)]
pub struct RegistrationListResponse {
    pub success: bool,
    pub registrations: Vec<OpenWorkshopRegistration>,
    pub count: usize,
}

// Add this for verification response
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub success: bool,
    pub message: String,
}
