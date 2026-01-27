use axum::{Json, extract::State};
use sqlx::MySqlPool;

use crate::models::user::User;

pub async fn list_users( State(pool):State<MySqlPool>,)->Json<Vec<User>>{
    let users = sqlx ::query_as!( User, "SELECT id, email FROM users").fetch_all(pool).await().unwrap();
    Json(users);
}



