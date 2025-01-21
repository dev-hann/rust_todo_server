use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header};
use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use jsonwebtoken::{decode, Validation, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub exp: usize,
}

const SECRETKEY: &str = "secret";

impl Claims {
    pub fn new(user_id: u32) -> Self {
        let exp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize + 3600; // 1 hour
        Claims { user_id, exp }
    }
    pub fn to_token(&self) -> String {
        let header = Header::default();
        let token = encode(&header, &self, &EncodingKey::from_secret(SECRETKEY.as_ref())).unwrap();
        token
    }
}

#[derive(Debug)]
pub struct AuthToken(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Authorization 헤더에서 토큰 추출
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                let validation = Validation::new(Algorithm::HS256);
                // 토큰 검증
                match decode::<Claims>(token, &DecodingKey::from_secret(SECRETKEY.as_ref()), &validation) {
                    Ok(token_data) => {
                        return request::Outcome::Success(AuthToken(token_data.claims));
                    },
                    Err(e) => return request::Outcome::Error((Status::Unauthorized, Status::UnavailableForLegalReasons)),
                }
            }
        }
        request::Outcome::Error((Status::Unauthorized, Status::UnavailableForLegalReasons))
    }
}

#[rocket::async_trait]
impl Fairing for AuthToken {
    fn info(&self) -> Info {
        Info {
            name: "AuthToken",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &Request<'_>, response: &mut Response<'_>) {
        println!("Request: {:?}", request);
        println!("Response: {:?}", response);
    }
}
