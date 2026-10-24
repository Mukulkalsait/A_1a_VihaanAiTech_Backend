// FILE: ./src/app/app_state.rs
use sqlx;

use crate::config::AppConfig; // .env struct

#[derive(Clone)]
/// AppState
/// * Config => Appconfig(.env file)
/// * db = SqlitePool
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::SqlitePool,
}
