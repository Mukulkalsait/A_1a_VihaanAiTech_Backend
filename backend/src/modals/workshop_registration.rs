use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::modals::{users_mod::UserID, workshop::WorkshopID};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WorkshopRegistration {
    pub registration_id: i32,
    pub user_id: Option<UserID>,

    pub workshop_id: Option<WorkshopID>,
    pub workshop_name: String,
    pub workshop_date: String,
    pub amount: f64,
    pub payment_screenshot_path: Option<String>,

    pub payment_status: String,
    pub registration_status: String,
    pub attended: i32, // sqlite 1/0

    pub attended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,

    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkshopRegisterResponse {
    pub success: bool,
    pub message: String,
    pub registration_id: i32,
}

#[derive(Debug, Serialize)]
pub struct SingleRegistrationResponse {
    pub success: bool,
    pub registration: WorkshopRegistration,
}

#[derive(Debug, Serialize)]
pub struct RegistrationActionResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationListResponse {
    pub success: bool,
    pub count: usize,
    pub registrations: Vec<WorkshopRegistration>,
}
