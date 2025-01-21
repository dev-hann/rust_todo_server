use rocket::serde::json::Json;
use rocket::State;
use crate::models::app_state::AppState;
use crate::models::user::User;
use rocket::http::Status;
use crate::models::claims::Claims;

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