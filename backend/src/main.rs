// FILE: /src/main.rs
mod app;
mod config;
mod errors;
mod handlers;
mod modals;
mod utils;

#[tokio::main]
/// RUNTIME ENTRY.
/// anyhow::Resualt<()>
/// resualt -> <Ok(), Err()> | But for generic type => (error? if yes type?)
/// anyhow allow us to use ? early return. => give anyting to return as resualt.
async fn main() -> anyhow::Result<()> {
    // this allow to print tracing::info! /error! /warn! 🖲️
    tracing_subscriber::fmt().with_env_filter("debug").init();

    let config = config::AppConfig::from_env()?; // dot env file
    let db = sqlx::sqlite::SqlitePoolOptions::new().max_connections(6).connect(&config.database_url).await?; // ACTUAL POOL CREATION WITH 6 connections.
    let app = app::build_app(config.clone(), db.clone());

    tracing::info!("Running database migrations...");
    sqlx::migrate!().run(&db).await?; // Y: DB auto Migrations.
    tracing::info!("Database migrations complete.");

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    // await can ONLY used on something that might take time. eg. Network, socket, timers, file I/O
    // listner say 'i need this port 👆', OS can say => {delay, fail, wait for other resources} ➡️ await.
    // await suspends the current async task until the awaited future makes progress
    // Y: await : So the function is paused till OS binds the port.

    println!("⚡ VAT on {}", config.server_addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
