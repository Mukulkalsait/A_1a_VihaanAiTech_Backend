use axum::{Router, routing::get};

pub fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "OK"
}
