use serde::{Deserialize, Serialize};

pub type UserID = uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub mobile: Option<String>,
    pub dob: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: UserID,
    pub first_name: String,
    pub email: String,
}

pub struct UserResponse {
    pub id: UserID,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub picture: Option<String>,
    pub membership: String,
}
