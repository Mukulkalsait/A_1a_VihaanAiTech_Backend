
use crate::errors::ApiError;

pub async fn fail()-> Result<&'static str, ApiError> {
    Err(ApiError::Unauthorized)
}
