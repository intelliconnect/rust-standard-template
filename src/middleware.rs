use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::request::Parts,
    response::Response,
};
use crate::{model::auth_models::Claims, KEYS};
use jsonwebtoken::{decode, Validation};
use chrono::Utc;

fn forbidden(msg: &str) -> Response<Body> {
    Response::builder()
        .status(403)
        .body(Body::from(msg.to_string()))
        .unwrap()
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(parts: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| forbidden("Missing or invalid Authorization header"))?;

        let data = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| forbidden("Invalid token"))?;

        let claims = data.claims;

        if let Some(exp) = claims.exp {
            if exp < Utc::now().timestamp() as f64 {
                return Err(forbidden("Token expired"));
            }
        }

        Ok(claims)
    }
}
