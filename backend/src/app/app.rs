// FILE: ./src/app/app.rs

use axum::{Router, routing};
use tower_http::cors::{Any, CorsLayer};

// EXT
use crate::app::AppState;
use crate::config;
use crate::handlers::{create_user, google_auth, list_user, me, mobile_login, mobile_register, test};

// INT

/// IMP: Router::new()...route().with_state(state) => tells axum that
/// Every request can access =>
///     - State(state): State<AppState> directly anywhere.
///     - even inside functions parameters.
/// THEREFORE=>
///     - DB from this state can be passed into function.
///
pub fn build_app(config: config::AppConfig, db: sqlx::SqlitePool) -> axum::Router {
    let state = AppState { config, db }; // cofig + db.pull 

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    Router::new()
        .route("/home", routing::get(homepageurl))
        // important
        .route("/auth/google", routing::post(google_auth::google_auth))
        .route("/me", routing::get(me))
        .route("/auth/register", routing::post(mobile_register))
        .route("/auth/phone-login", routing::post(mobile_login))
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(list_user))
        .route("/appx", routing::get(appx))
        // fail
        .route("/fail", routing::get(test::fail))
        .layer(cors)
        .with_state(state)
}

async fn homepageurl() -> &'static str {
    "<html><h1>Welcome to homepage</h1></html>"
}
async fn appx() -> &'static str {
    "appx"
}
