use serde::{Deserialize, Serialize};
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::request::Request;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Data;
use rocket::response::Response;
use rocket::http::Status;
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
pub struct AuthToken;

#[rocket::async_trait]
impl Fairing for AuthToken {
    fn info(&self) -> Info {
        Info {
            name: "AuthToken",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let path = req.uri().path();
        if path.starts_with("/api/auth") {
            println!("Skipping authentication for /api/auth path");
            return;
        }
        let header = req.headers().get_one("Authorization");
        match header {
            Some(auth_header) => {
                if auth_header.starts_with("Bearer ") {
                    let token = auth_header.split_whitespace().nth(1).expect("Bearer token expected");
                    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRETKEY.as_ref()), &Validation::default());
                    println!("Token data: {:?}", token_data);
                }
            }
            None => {
                req.local_cache(|| Some(rocket::http::Status::Unauthorized));
            }
        }

    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        println!("response {:?}", res);
        println!("request {:?}", req);
    }
}