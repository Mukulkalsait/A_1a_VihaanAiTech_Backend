use axum::{Router,routing::get};
use crate::handlers::users;

pub fn routes() -> Router{
    Router::new().route("/", get(users::list_users))
}
