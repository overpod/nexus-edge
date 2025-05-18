use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: String) -> Self {
        let exp = env::var("JWT_EXPIRED_IN_DAYS").expect("JWT_EXPIRED_IN_DAYS env var not set");
        let iat = Utc::now();

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: (iat + Duration::days(exp.parse::<i64>().unwrap())).timestamp(),
        }
    }
}

pub fn sign(id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET env var not set");
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET env var not set");
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
