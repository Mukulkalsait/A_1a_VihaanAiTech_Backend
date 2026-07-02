// FILE: ./src/app/app.rs

use axum::{Router, routing, routing::{post, get, delete}};
use tower_http::cors::{Any, CorsLayer};

// EXT
use crate::app::AppState;
use crate::config;
use crate::handlers::workshop::open_workshop_handler::*;
use crate::handlers::{create_user, login_handler::google_auth, list_user, me, mobile_login, mobile_register, fail};

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
        // .route("/home", routing::get(homepageurl)) R: exaple of home
        // important
        .route("/auth/google", routing::post(google_auth::google_auth))
        .route("/me", routing::get(me))
        .route("/auth/register", routing::post(mobile_register))
        .route("/auth/phone-login", routing::post(mobile_login))
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(list_user))
        .route("/open-workshop/register", post(register_open_workshop))
        .route("/open-workshop/registrations", get(list_registrations))     // ← get works now
        .route("/open-workshop/registration/{id}", get(get_registration))
        .route("/open-workshop/verify/{id}", post(verify_registration))
        .route("/open-workshop/registration/{id}", delete(delete_registration))
        // fail
        .route("/fail", routing::get(fail::fail))
        .layer(cors)
        .with_state(state)
}

async fn _homepageurl() -> &'static str {
    "Welcome to homepage"
}
async fn _appx() -> &'static str {
    "appx"
}
