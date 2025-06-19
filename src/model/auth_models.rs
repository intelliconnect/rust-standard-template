use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub family_name: Option<String>,
    pub name: Option<String>,
    pub unique_name: Option<String>,
    pub exp: Option<f64>,
    pub iat: Option<f64>,
    pub token_status: Option<String>,
}
#[allow(dead_code)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
