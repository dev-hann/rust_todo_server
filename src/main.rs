#[macro_use]
extern crate rocket;

mod routers;
mod models;
mod handlers;

#[launch]
fn rocket() -> _ {
    rocket::build().manage(models::app_state::AppState::new())
        .mount("/api",routers::todo::routes())
}
