// FILE: ./src/db/init.rs

use sqlx::{self, SqlitePool};
// TAG: EXTERMAL ===============

// use crate::app::app_state::AppState;

pub async fn init_db(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let create_user_table_query = r#"CREATE TABLE IF NOT EXISTS core_users(
    user_id INTEGER PRIMARY KEY,

    user_first_name TEXT NOT NULL,
    user_last_name TEXT NOT NULL,
    user_mobile TEXT ,
    user_dob TEXT,
    user_email TEXT NOT NULL,

    user_verified INTEGER NOT NULL DEFAULT 0,
    user_password TEXT NOT NULL,

    user_picture TEXT,
    user_login_method TEXT,

    user_verification_token TEXT,
    user_token_expires_at TEXT,

    user_role TEXT NOT NULL DEFAULT 'user' CHECK (user_role IN ('admin', 'user')),

    user_created_at TEXT DEFAULT CURRENT_TIMESTAMP, 
    user_updated_at TEXT DEFAULT CURRENT_TIMESTAMP

    );"#;

    // default timestamp in sqlite with CURRENT_TIMESTAMP =>  "2026-05-05 20:30:00"
    //
    // Y:  report database.
    // report_data Text,
    // score INTEGER,
    // pdf_path TEXT,
    // created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    // sent_status INTIGER DEFAULT 0

    sqlx::query(create_user_table_query).execute(pool).await?;
    Ok(())
}
