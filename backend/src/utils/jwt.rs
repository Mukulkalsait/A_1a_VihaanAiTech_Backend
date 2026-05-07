/// FILE: /src/utils/jwt.rs
use jsonwebtoken::{EncodingKey, Header, encode};
use serde;

use crate::errors::ApiError;

//-----------------------------------------------------------------------------------------
#[derive(serde::Deserialize, serde::Serialize)]
/// - (CLAIM=PAYLOAD) CONTAINS => (sub(subject), email, expiry)
/// - Header | Payload | Signature
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(id: i64, user_email: String, expiration_time: usize) -> Self {
        Claims { sub: id, email: user_email, exp: expiration_time }
    }
}

/// # JWT (Very similar to SSH private signing key conceptually)
///
/// ## WORKFLOW:
/// - Google token => verify with Google => backend trusts user
/// - backend creates ITS OWN JWT => frontend stores YOUR JWT => future requests use YOUR JWT
///
/// ### actual working happning.
///  1. expiration -> chrono time now as usize.
///  2. Claims::new() => put into struct to SerDe
///  3. Encode function run.
///
/// #### fn jsonwebtoken::encoding::encode() explanation:
/// ```rust
/// pub fn encode<T>(header: &Header, claims: &T, key: &EncodingKey) -> Result<String>
/// where
///     T: Serialize,
/// ```
/// > contains header, claims, and key.
///  1. header::default() default-header =HS256 , claim (data), secrete (token)
///  2. clamis -> our struct -> Serde -> data.
///  3. &EncodingKey::from_secret(secret.as_ref()) -> encodign technique to hide the key.
///  - here "secret" is the attribute passed down from .evn file which is our JWT token.
///
pub fn generate_jwt(user_id: i64, user_email: String, secret: &str) -> Result<String, ApiError> {
    // checked_add_signed => 24 hr safety. timestamp => convert into unix timestamp...
    let expiration = chrono::Utc::now().checked_add_signed(chrono::Duration::hours(24)).unwrap().timestamp() as usize;
    let claims = Claims::new(user_id, user_email, expiration);

    // encode-> create jwt string from data.
    // { "alg": "HS256", "typ": "JWT" }
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| ApiError::Internal)
}
