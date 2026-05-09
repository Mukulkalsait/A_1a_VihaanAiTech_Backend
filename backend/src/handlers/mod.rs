// FILE: /src/handlers/mod.rs

pub mod google_auth;
pub mod me_auth;
pub mod mobile_auth;
pub mod test;
pub mod user;
pub mod user_stractures;

// Re-export
pub use me_auth::me;
pub use mobile_auth::{mobile_login, mobile_register};
pub use user::create_user;
pub use user::list_user;
