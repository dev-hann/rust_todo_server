use rocket::Route;
use crate::handlers::auth::{get_users, login, register,get_user};

    pub fn routes() -> Vec<Route> {
        routes![login,register,get_users,get_user]
    }