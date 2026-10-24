use serde::{Deserialize, Serialize};

use crate::modals::users_mod::UserID;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: UserID,
    pub user_email: String,
    pub user_first_name: String,
    pub user_picture: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct ExcitingUser {
    pub id: UserID,
    pub user_email: String,
    pub user_first_name: String,
    pub user_picture: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ExcitingUserID {
    pub id: UserID,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub user_id: UserID,
    pub name: String,
    pub email: String,
    pub picture: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserClaims {
    pub user_id: UserID,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Deserialize)]
/// #### Googel Login Workflow
///
/// - Frontend => Google Login Popup
/// - Google returns ID Token => Rrontend sends token to backend => POST /auth/google
/// - Backend verifies token with Google => Backend finds/creates user
/// - Backend creates JWT => JWT returned to frontend => Frontend stores JWT
/// - Future requests send JWT
pub struct GoogleAuthRequest {
    pub token: String,
}
