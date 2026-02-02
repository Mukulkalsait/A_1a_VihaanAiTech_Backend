
use axum::{Router, routing::get};

pub fn build_app()-> Router{
    Router::new()
        .route("/appx", get(appx))
        .route("/health", get(health))
}

async fn health()->&'static str{"ok"}
async fn appx()->&'static str {"even this works"}
