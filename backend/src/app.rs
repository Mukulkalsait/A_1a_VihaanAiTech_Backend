
use axum::{Router, routing::get};
use crate::handlers::test;

pub fn build_app()-> Router{
    Router::new()
        .route("/appx", get(appx))
        .route("/health", get(health))
        .route("/fail",get(test::fail))
}

async fn health()->&'static str{"ok"}
async fn appx()->&'static str {"even this works"}
