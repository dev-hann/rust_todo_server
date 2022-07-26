#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    // Todo Server ReadMe.
    return "Hello, world!";
}

#[get("/api/todo/<index>")]
fn load_todo(index: i32) -> String {
    return format!("Hello {}", index);
}

#[get("/api/todoList")]
fn load_todo_list() {}

#[put("/api/todo")]
fn update_todo() {}

#[delete("/api/todo")]
fn remove_todo() {}




#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, load_todo])
}
