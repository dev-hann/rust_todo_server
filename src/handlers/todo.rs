use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::models::app_state::AppState;
use crate::models::todo::Todo;
use crate::service::database;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todos", format = "json")]
 pub fn todos(state: &State<AppState>) -> Result<Json<Vec<Todo>>, Status> {
    let todo_list = state.inner().database.get_all_todos();
    Ok(Json(todo_list.clone()))
}

#[post("/todos", format = "json", data = "<todo>")]
 pub fn create_todo(state: &State<AppState>, todo: Json<Todo>) -> Result<Json<Todo>, Status> {
    let mut todo_list = state.inner().todo_list.lock().unwrap();
    let todo = todo.into_inner();
    todo_list.push(todo.clone());
    Ok(Json(todo.clone()))
 }

#[delete("/todos/<id>")]
 pub fn delete_todo(state: &State<AppState>, id: u32) -> Result<Json<bool>, Status> {
    let mut todo_list = state.inner().todo_list.lock().unwrap();
    todo_list.retain(|todo| todo.id != id);
    Ok(Json(true))
}

#[patch("/todos/<id>", format = "json", data = "<todo>")]
pub fn update_todo(state: &State<AppState>, id: u32, todo: Json<Todo>) -> Result<Json<Todo>, Status> {
    let mut todo_list = state.inner().todo_list.lock().unwrap();
    let todo = todo.into_inner();
    todo_list.retain(|todo| todo.id != id);
    Ok(Json(todo.clone()))
}