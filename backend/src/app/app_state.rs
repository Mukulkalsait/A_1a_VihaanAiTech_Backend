use sqlx;
//EXT:---------------
use crate::config::AppConfig; // .env struct
//INT:---------------

#[derive(Clone)]
/// AppState
/// * Config => Appconfig(.env file)
/// * db = SqlitePool
///
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::SqlitePool,
}
