// FILE: ./src/app/app.rs

use axum::{
    Router,
    routing,
    routing::{delete, get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    app::AppState,
    config,
    handlers::{
        admin::*,
        create_user,
        email_login,
        email_register,
        fail,
        list_user,
        login_handler::google_auth,
        me,
        mobile_login,
        mobile_register,
        workshop::{open_workshop_handler::*, workshop_handler::*, workshop_registrations_handler::*},
    },
};

/// #  Router::new()...route().with_state(state) => tells axum that
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
        // users and auth
        .route("/auth/google", routing::post(google_auth::google_auth))
        .route("/me", routing::get(me))
        .route("/auth/register-phone", routing::post(mobile_register))
        .route("/auth/phone-login", routing::post(mobile_login))
        .route("/auth/register-email", routing::post(email_register))
        .route("/auth/email-login", routing::post(email_login))
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(list_user))
        // Workshops All
        .route("/workshops", get(get_all_workshops))
        .route("/admin/workshops", post(new_workshop))
        .route("/workshops/{slug}", get(get_workshop_by_slug))
        // Workshops that are open for registrations
        .route("/open-workshop/register", post(register_open_workshop))
        // Workshop Registrations
        .route("/workshop/registrations", get(list_all_registrations))
        .route("/workshop/registration/{id}", get(get_registration))
        .route("/workshop/registration/{id}/verify", post(verify_registration))
        .route("/workshop/registration/{id}", delete(delete_registration))
        // fail
        .route("/fail", routing::get(fail::fail))
        .route("/forbiden", routing::get(fail::forbiden))
        // admin
        .route("/admin/whoami", routing::get(whoami)) // TEST:: testroute
        .layer(cors)
        .with_state(state)
}

async fn _homepageurl() -> &'static str { "Welcome to homepage" }
async fn _appx() -> &'static str { "appx" }
