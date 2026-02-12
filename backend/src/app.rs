
use axum::{Router, routing::get};
use crate::handlers::test::{self, fail};
use crate::app_state::AppState;
use crate::config::AppConfig;


pub fn build_app(config:AppConfig)-> Router{
    let state = AppState {config};
    Router::new()
        .route("/health", get(health))
        .with_state(state)
        .route("/appx", appx())

}

async fn health()->&'static str{"ok"}
async fn appx()->&'static str {"even this works"}


