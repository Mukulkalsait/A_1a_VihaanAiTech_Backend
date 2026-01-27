use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use crate::config;

pub async fn init_db() -> anyhow::Result<MySqlPool> {
    let pool = MySqlPoolOptions::new().max_connections(10).connect(&config::database_url()).await?;
    OK(pool)
}
