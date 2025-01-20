#[macro_use]
extern crate rocket;

mod routers;
mod handlers;
mod models;
mod service;



#[launch]
fn rocket() -> _ {
    rocket::build().manage(models::app_state::AppState::new())
        .mount("/api",routers::todo::routes())
}
