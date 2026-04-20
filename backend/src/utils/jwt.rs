use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

//-----------------------------------------------------------------------------------------
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claim {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: i64, user_email: String, secret: &str) -> String {
    let expiration = chrono::Utc::now().checked_add_signed(chrono::Duration::hours(24)).unwrap().timestamp() as usize;
    // checked_add_signed => 24 hr safety. timestamp => convert into unix timestamp...
    let claims = Claim { sub: user_id, email: user_email, exp: expiration };
    // (CLAIM=PAYLOAD) CONTAINS => (sub(subject), email, expiry)
    // Header | Payload | Signature
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    // encode-> create jwt string from data.
    // default-header =HS256 , claim (data), secrete (token)
    // { "alg": "HS256", "typ": "JWT" }
}
