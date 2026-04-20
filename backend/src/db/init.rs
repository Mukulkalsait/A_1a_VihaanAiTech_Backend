use sqlx::{self, SqlitePool};
// TAG: EXTERMAL ===============

// use crate::app::app_state::AppState;

pub async fn init_db(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let create_user_table_query = r#"CREATE TABLE IF NOT EXISTS users(
    id TEXT PRIMARY KEY,
    user_name TEXT NOT NULL,
    user_mobile TEXT ,
    user_dob TEXT,
    user_email TEXT,
    report_data Text,
    score INTIGER,
    pdf_path TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    sent_status INTIGER DEFAULT 0
    );"#;

    sqlx::query(create_user_table_query).execute(pool).await?;
    Ok(())
}
