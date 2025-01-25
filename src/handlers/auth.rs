use crate::models::app_state::AppState;
use crate::models::user::User;
use crate::models::claims::Claims;

use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::request::Request;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Data;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[post("/login", format = "json", data = "<user>")]
pub fn login(state: &State<AppState>, user: Json<User>) -> Result<Json<String>, Status> {
    let user = user.into_inner();
    let user_data = state.inner().auth_db.get_user_by_name(user.get_name());
    match user_data {
        Some(db_user) if db_user.get_password() == user.get_password() => {
            let claims = Claims::new(db_user.get_id());
            let token = claims.to_token();
            Ok(Json(token))
        }
        _ => Err(Status::Unauthorized),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn register(state: &State<AppState>, user: Json<User>) -> Result<Json<User>, Status> {
    let user = user.into_inner();
    state.inner().auth_db.create_user(user.clone()).map_err(|_| Status::InternalServerError)?;
    Ok(Json(user.clone()))
}

#[get("/users", format = "json")]
pub fn get_users(state: &State<AppState>) -> Result<Json<Vec<User>>, Status> {
    let users = state.inner().auth_db.get_all_users();
    Ok(Json(users.clone()))
}

#[get("/users/<id>", format = "json")]
pub fn get_user(state: &State<AppState>, id: u32) -> Result<Json<User>, Status> {
    let user = state.inner().auth_db.get_user(id);
    match user {
        Some( user) => Ok(Json(user.clone())),
        None => Err(Status::NotFound),
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
                    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(crate::consts::SECRETKEY.as_ref()), &Validation::default());
                    println!("Token data: {:?}", token_data);
                }
            }
            None => {
                req.local_cache(|| Some(rocket::http::Status::Unauthorized));
            }
        }

    }
}