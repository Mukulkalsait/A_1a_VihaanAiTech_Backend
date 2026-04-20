use axum::{Router, routing};

// EXT
use crate::app;
use crate::app::AppState;
use crate::config;
use crate::handlers::user::list_user;
use crate::handlers::{create_user, google_auth, test};

//ENT

pub fn build_app(config: config::AppConfig, db: sqlx::SqlitePool) -> axum::Router {
    let state = AppState { config, db };
    Router::new()
        .route("/a", routing::get(homepageurl))
        .route("/fail", routing::get(test::fail))
        .route("/auth/google", routing::post(google_auth::google_auth))
        .route("/appx", routing::get(appx))
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(list_user))
        .with_state(state)
}

async fn homepageurl() -> &'static str {
    "ok"
}
async fn appx() -> &'static str {
    "appx"
}
