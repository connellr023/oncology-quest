use crate::models::client_user::ClientUser;
use std::{future::{ready, Ready}, env::var};
use actix_web::{dev::Payload, error::{Error, ErrorUnauthorized}, FromRequest, HttpRequest};
use jsonwebtoken::{encode, decode, EncodingKey, Header, DecodingKey, Validation, Algorithm};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};

const HASH_ALGORITHM: Algorithm = Algorithm::HS256;
const EXPIRATION_WEEKS: i64 = 1;

#[inline(always)]
fn secret_key() -> String {
    var("JWT_SECRET").expect("Expected JWT secret key.")
}

#[derive(Serialize, Deserialize)]
pub struct JwtUserClaim {
    pub sub: ClientUser,
    pub exp: i64
}

impl JwtUserClaim {
    pub fn encode(user: ClientUser) -> String {
        let exp = (Utc::now() + Duration::weeks(EXPIRATION_WEEKS)).timestamp();

        encode(
            &Header::new(HASH_ALGORITHM),
            &JwtUserClaim {
                sub: user,
                exp
            },
            &EncodingKey::from_secret(secret_key().as_ref())
        ).unwrap()
    }
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
                        &DecodingKey::from_secret(secret_key().as_ref()),
                        &Validation::new(HASH_ALGORITHM),
                    );

                    return match token_data {
                        Ok(data) => {
                            if data.claims.exp < Utc::now().timestamp() {
                                return ready(Err(ErrorUnauthorized("Token has expired.")));
                            }

                            ready(Ok(data.claims))
                        },
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