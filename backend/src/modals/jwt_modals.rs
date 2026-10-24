//-----------------------------------------------------------------------------------------
use serde;

use crate::modals::users_mod::UserID;

#[derive(serde::Deserialize, serde::Serialize)]
/// - (CLAIM=PAYLOAD) CONTAINS => (sub(subject), email, expiry)
/// - Header | Payload | Signature
pub struct Claims {
    pub sub: UserID,
    pub email: String,
    pub exp: usize,
}
