use std;

use tokio;
// EXT---------------------

mod app;
mod config;
// use crate::config::env::AppConfig;
mod db;
mod errors;
mod handlers;
mod utils;
// INT---------------------

#[tokio::main]
/// RUNTIME ENTRY.
/// anyhow::Resualt<()>
/// resualt -> <Ok(), Err()> | But for generic type => (error? if yes type?)
/// anyhow allow us to use ? early return. => give anyting to return as resualt.
async fn main() -> anyhow::Result<()> {
    let config = config::AppConfig::from_env()?;
    let db = sqlx::sqlite::SqlitePoolOptions::new().max_connections(6).connect(&config.database_url).await?;
    let app = app::build_app(config.clone(), db.clone());

    db::init::init_db(&db).await?;

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    // await can ONLY used on something that might take time. eg. Network, socket, timers, file I/O
    // listner say 'i need this port 👆', OS can say => {delay, fail, wait for other resources} ➡️ await.
    // await suspends the current async task until the awaited future makes progress
    // Y: await
    // G: So the function is paused till OS binds the port.

    println!("⚡ VAT on {}", &config.server_addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
//IMP:===============================| Info of Structs |=====================================================
//
// 1. AppConfig (config/env) | envFileDetails.
// 2. AppState (config + db)
// 3. ApiError (Errors )
// 4. /db/init db_init() | db_query  sqlx::query().exicute(pool).await?
// 5. GoogleAuth
// 6. App::build_app () -> Router |  Router::new .... (AppState)
