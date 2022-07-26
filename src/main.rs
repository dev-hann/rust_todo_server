#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "Hello, world!";
}

#[get("/api/todo/<index>")]
fn load_todo(index: i32) -> String {
    return format!("Hello {}", index);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, load_todo])
}
