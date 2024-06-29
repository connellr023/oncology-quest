use crate::models::client_user::ClientUser;
use std::future::{ready, Ready};
use actix_web::{dev::Payload, error::{Error, ErrorUnauthorized}, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

#[derive(Serialize, Deserialize)]
pub struct JwtUserClaim {
    pub sub: ClientUser,
    pub exp: usize
}

impl FromRequest for JwtUserClaim {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ");
                    let token_data = decode::<JwtUserClaim>(
                        token,
                        &DecodingKey::from_secret("your_secret_key".as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    return match token_data {
                        Ok(data) => ready(Ok(data.claims)),
                        Err(_) => ready(Err(ErrorUnauthorized("Invalid token."))),
                    };
                }
            }
        }

        ready(Err(ErrorUnauthorized("No authorization header found.")))
    }
    
    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}