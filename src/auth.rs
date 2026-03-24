use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::Utc;
use crate::{error::AppError, model::Claims};

pub fn create_token(username: &str, role: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: username.to_string(),
        role: role.to_string(),
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as u64,
    };

    let secret = &crate::config::CONFIG.server.jwt_secret;
    let key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::default(), &claims, &key)
        .map_err(|_| AppError::InternalError)
}