// FILE: /src/handlers/mod.rs
pub mod admin;
pub mod fail;
pub mod login_handler;
pub mod users;
pub mod workshop;

pub use login_handler::{
    email_auth::{email_login, email_register},
    me_auth::me,
    mobile_auth::{mobile_login, mobile_register},
};
pub use users::user::{create_user, list_user};
