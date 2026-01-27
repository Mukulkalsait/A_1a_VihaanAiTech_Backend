
mod app;
mod config;
mod db;
mod routes;

use axum::serve::Listener;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();

    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::from_default_env()).with(tracing_subscriber::fmt::layer()).init();

    let app = app::create_app().await?;

    let addr = format!("0.0.0.0:{}",std::env::var("APP_PORT").unwrap_or("3000".into()));

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("⚡VAT Backend running on http://{}",addr);

    axum::serve(listener,app).await?;

    Ok(())
}
