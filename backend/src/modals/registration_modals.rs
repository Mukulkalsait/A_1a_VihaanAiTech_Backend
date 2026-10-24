use crate::modals::users_mod::UserID;

// Mobile Registrations. ----------------------------------------------------------------------------------------------------
#[derive(serde::Deserialize)]
pub struct MobileRegisterRequest {
    pub name: String,
    pub mobile: String,
}

#[derive(serde::Deserialize)]
pub struct MobileLoginRequest {
    pub mobile: String,
}

#[derive(sqlx::FromRow)]
pub struct MobileUser {
    pub id: UserID,
    pub name: String,
    pub mobile: String,
}

// Email Registrations. ----------------------------------------------------------------------------------------------------

#[derive(serde::Deserialize)]
pub struct EmailRegisterRequest {
    pub name: String,
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct EmailLoginRequest {
    pub email: String,
}

#[derive(sqlx::FromRow)]
pub struct EmailUser {
    pub id: UserID,
    pub name: String,
    pub email: String,
}
