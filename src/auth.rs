use crate::config::AppState;
use jwt::{DecodingKey, EncodingKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use jsonwebtoken as jwt;

use crate::config;

const USED_ALGO: jwt::Algorithm = jwt::Algorithm::HS512;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl Auth {
    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_secret(secret);
        jwt::encode(&jwt::Header::new(USED_ALGO), self, &encoding_key).expect("jwt")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        if let Some(auth) = extract_auth_from_request(req, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(request_header: &str) -> Option<&str> {
    if request_header.starts_with(config::TOKEN_PREFIX) {
        Some(&request_header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Validation};

    let decoding_key = DecodingKey::from_secret(secret);

    jwt::decode(
        token,
        &decoding_key,
        &Validation::new(USED_ALGO),
    )
    .map_err(
        |err| {
            eprintln!("Auth decode error: {:?}", err)
        }
    )
        .ok()
        .map(|token_data| token_data.claims)
}