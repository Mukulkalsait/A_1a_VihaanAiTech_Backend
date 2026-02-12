
 use crate::errors::ApiError;
 use crate::app_state::AppState;
 use axum::extract::State;

 pub async fn fail( State(state): State<AppState>, )-> Result<&'static str, ApiError> {
     println!("ENV = {}",state.config.app_env);
     Err(ApiError::Unauthorized)
 }

pub async fn forbid()-> Result<&'static str, ApiError>{
    Err(ApiError::Forbiden)
}
