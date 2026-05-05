use axum::{Router, routing};

// EXT
use crate::app;
use crate::app::AppState;
use crate::config;
use crate::handlers::{create_user, google_auth, list_user, me, test};

//ENT

pub fn build_app(config: config::AppConfig, db: sqlx::SqlitePool) -> axum::Router {
    let state = AppState { config, db };
    Router::new()
        .route("/home", routing::get(homepageurl))
        // important
        .route("/auth/google", routing::post(google_auth::google_auth))
        .route("/me", routing::get(me))
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(list_user))
        .route("/appx", routing::get(appx))
        // fail
        .route("/fail", routing::get(test::fail))
        .with_state(state)
}

async fn homepageurl() -> &'static str {
    "ok"
}
async fn appx() -> &'static str {
    "appx"
}
