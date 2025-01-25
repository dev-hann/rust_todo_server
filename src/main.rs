#[macro_use]
extern crate rocket;

mod routers;
mod handlers;
mod models;
mod service;
mod consts;

use handlers::auth::AuthToken;


#[launch]
fn rocket() -> _ {
    rocket::build().manage(models::app_state::AppState::new())
        .attach(AuthToken)
        .mount("/api/todo",routers::todo::routes())
        .mount("/api/auth",routers::auth::routes())
}
