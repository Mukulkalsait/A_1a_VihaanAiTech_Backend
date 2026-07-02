use std::sync::Arc;

use axum::extract::{multipart, Multipart, State};
use axum::{Extension, Json};
use reqwest::StatusCode;
use tracing::field;
use tracing_subscriber::field::debug;

use crate::app::AppState;
use crate::modals::workshop_mod::*;

#[derive(Debug, Clone)]
pub struct UserClaims {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

// pub async fn register_for_workshop(
//     State(state): State<Arc<AppState>>,
//     Extension(user_claim): Extension<UserClaims>,
//     mut multipart:Multipart,
// )
// -> Result<Json<serde_json::Value>,(StatusCode::Json<serde_json::Value>)>
// {
//     let mut workshop_name = String::from("Data Science Workshop");
//     let mut workshop_date = String::from("2026-06-07");
//     let mut screenshot_path = None;
//
//     while let Some(field) =  multipart.next_field().await.map_err(|e| {
//         (
//             StatusCode::BAD_REQUEST,
//             Json(serde_json::json!( { "success": false, "message": format!("Faild to parse the data {}", e), }) )
//         )
//     })? {
//         let field_name = field.name().unwrap_or("").to_string();
//
//         match field_name.as_str(){
//             "workshop_name" =>  if let Ok(data) = field.text().await{workshop_name = data;}  ,
//             "workshop_data" => if let Ok(data) = field.text().await{workshop_date = data;}
//             "screenshot" =>  {
//                 let data = field.bytes().await.map_err(|e| {
//                 ( StatusCode::BAD_REQUEST,
//                         Json(serde_json::json!({
//                             "success" = false,
//                             "message"  = format!("faild to read screenshot {}",e)
//                         }))
//
//                 )
//                 })
//             }
//
//         }
//
//
//     }
//
//
// }
