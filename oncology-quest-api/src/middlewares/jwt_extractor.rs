use crate::models::client_user::ClientUser;
use std::{future::{ready, Ready}, env::var};
use actix_web::{dev::Payload, error::{Error, ErrorInternalServerError, ErrorUnauthorized}, http::header, FromRequest, HttpRequest};
use jsonwebtoken::{encode, decode, EncodingKey, Header, DecodingKey, Validation};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use anyhow::Result;

const EXPIRATION_WEEKS: i64 = 1;

#[inline(always)]
fn secret_key() -> String {
    var("JWT_SECRET").expect("Expected JWT secret key.")
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaim<T> {
    pub sub: T,
    pub exp: i64
}

pub type JwtUserClaim = JwtClaim<ClientUser>;

impl JwtClaim<String> {
    pub fn encode(user: ClientUser) -> String {
        let exp = (Utc::now() + Duration::weeks(EXPIRATION_WEEKS)).timestamp();

        encode(
            &Header::default(),
            &JwtClaim {
                sub: serde_json::to_string(&user).unwrap(),
                exp
            },
            &EncodingKey::from_secret(secret_key().as_ref())
        ).unwrap()
    }

    pub fn deserialize(self) -> Result<JwtClaim<ClientUser>> {
        Ok(JwtClaim {
            sub: serde_json::from_str(&self.sub)?,
            exp: self.exp
        })
    }
}

impl FromRequest for JwtClaim<ClientUser> {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(token) = auth_header.to_str() {
                let token_data = decode::<JwtClaim<String>>(
                    token,
                    &DecodingKey::from_secret(secret_key().as_ref()),
                    &Validation::default(),
                );

                return match token_data {
                    Ok(data) => {
                        if data.claims.exp < Utc::now().timestamp() {
                            return ready(Err(ErrorUnauthorized("Token has expired.")));
                        }

                        let claim = match data.claims.deserialize() {
                            Ok(claim) => claim,
                            Err(e) => return ready(Err(ErrorInternalServerError(e.to_string())))
                        };

                        ready(Ok(claim))
                    },
                    Err(e) => ready(Err(ErrorInternalServerError(e.to_string())))
                };
            }
        }

        ready(Err(ErrorUnauthorized("No authorization header found.")))
    }
    
    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}