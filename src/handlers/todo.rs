use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::models::app_state::AppState;
use crate::models::todo::Todo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todos", format = "json")]
 pub fn todos(state: &State<AppState>) -> Result<Json<Vec<Todo>>, Status> {
    let todo_list = state.inner().database.get_all_todos();
    Ok(Json(todo_list.clone()))
}

#[get("/todos/<id>", format = "json")]
pub fn get_todo(state: &State<AppState>, id: u32) -> Result<Json<Todo>, Status> {
    let todo = state.inner().database.get_todo(id);
    match todo {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(Status::NotFound),
    }
}

#[post("/todos", format = "json", data = "<todo>")]
 pub fn create_todo(state: &State<AppState>, todo: Json<Todo>) -> Result<Json<Todo>, Status> {
    let todo = todo.into_inner();
    state.inner().database.add_todo(todo.clone());
    Ok(Json(todo.clone()))
 }

#[delete("/todos/<id>")]
 pub fn delete_todo(state: &State<AppState>, id: u32) -> Result<Json<bool>, Status> {
    state.inner().database.delete_todo(id);
    Ok(Json(true))
}

#[patch("/todos/<id>", format = "json", data = "<todo>")]
pub fn update_todo(state: &State<AppState>, id: u32, todo: Json<Todo>) -> Result<Json<Todo>, Status> {
    let todo = todo.into_inner();
    state.inner().database.update_todo(todo.clone());
    Ok(Json(todo.clone()))
}
