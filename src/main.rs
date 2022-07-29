#[macro_use]
extern crate rocket;

mod controller;
mod routes;

use controller::controller::Controller;
use controller::todo::TodoController;

#[get("/")]
fn index() -> &'static str {
    // Todo Server ReadMe.
    return "Hello, world!";
}

#[get("/api/todo/<index>")]
fn load_todo(index: i32) -> String {
    return format!("Hello {}", index);
}

#[post("/api/todo")]
fn update_todo() {}

#[delete("/api/todo")]
fn remove_todo() {}

#[launch]
fn rocket() -> _ {
    let c = TodoController::new();

    rocket::build()
        .mount("/", routes![index, load_todo])
        .mount(c.base(), routes![controller::todo::load_todo])
}
