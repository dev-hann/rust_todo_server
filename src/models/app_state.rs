use std::sync::Mutex;
use crate::models::todo::Todo;

pub struct AppState {
    pub todo_list: Mutex<Vec<Todo>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { todo_list: Mutex::new(Vec::new()) }
    }
}