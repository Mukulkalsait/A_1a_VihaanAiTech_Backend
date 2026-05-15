#[derive(serde::Deserialize)]
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

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ExcitingUser {
    pub id: i64,
    pub user_email: String,
    pub user_first_name: String,
    pub user_picture: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ExcitingUserID {
    pub id: i64,
}
