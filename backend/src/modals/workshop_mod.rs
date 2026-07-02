use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct WorkshopRegistration {
    pub registration_id: i32,
    pub user_id: i32,
    pub workshop_name: String,
    pub workshop_date: String,
    pub amoutn: f64,
    pub payment_ss_path: Option<String>,
    pub payment_status: String,
    pub registration_status: String,
    pub attended: i32,
    pub attended_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)] // coming so Deserialize
pub struct WorkshopRegistrationResponse {
    pub success: bool,
    pub message: String,
    pub registration_id: Option<i32>,
    pub user_info: Option<UserInfo>,
}

#[derive(Debug, Serialize)] // we are sending this so we serialize.
pub struct WorkshopRegistrationRequest {
    pub workshop_name: Option<String>,
    pub workshop_date: Option<String>,
}
