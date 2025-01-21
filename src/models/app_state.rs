use crate::service::todo_db;
use crate::service::auth_db;
pub struct AppState {
    pub todo_db: todo_db::TodoDatabase,
    pub auth_db: auth_db::AuthDatabase,
}

impl AppState {
    pub fn new() -> Self {
        let todo_db = todo_db::TodoDatabase::new();
        let auth_db = auth_db::AuthDatabase::new();
        AppState { todo_db, auth_db }
    }
}