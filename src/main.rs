#[macro_use]
extern crate rocket;

mod routers;
mod handlers;
mod models;
mod service;
use models::claims::AuthToken;

#[get("/protected")]
async fn protected_route(_auth: AuthToken) -> &'static str {
    "This is a protected route!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(models::app_state::AppState::new())
        .mount("/api/todo", routes![protected_route])
        .mount("/api/todo",routers::todo::routes())
        .mount("/api/auth",routers::auth::routes())
        .attach(AuthToken)
}
