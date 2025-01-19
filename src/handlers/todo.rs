use crate::models::app_state::AppState;
use crate::models::todo::Todo;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todos", format = "json")]
 pub fn todos(state: &State<AppState>) -> Result<Json<Vec<Todo>>, Status> {
    let todo_list = state.inner().todo_list.lock().unwrap();
    Ok(Json(todo_list.clone()))
}

#[post("/todos", format = "json", data = "<todo>")]
 pub fn create_todo(state: &State<AppState>, todo: Json<Todo>) -> Result<Json<bool>, Status> {
    let mut todo_list = state.inner().todo_list.lock().unwrap();
    todo_list.push(todo.into_inner());
    Ok(Json(true))
}

#[delete("/todos/<id>")]
 pub fn delete_todo(state: &State<AppState>, id: u32) -> Result<Json<bool>, Status> {
    let mut todo_list = state.inner().todo_list.lock().unwrap();
    todo_list.retain(|todo| todo.id != id);
    Ok(Json(true))
}
