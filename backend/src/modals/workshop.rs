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
}
