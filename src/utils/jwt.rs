use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::schemas::user_auth::Claims;

pub fn generate_jwt(user_id: i64, secret: &str) -> String {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims{
        id: user_id, 
        exp: exp
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .expect("Jwt encode failed")
}