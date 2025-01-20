use crate::service::database;

pub struct AppState {
    pub database: database::Database,
}

impl AppState {
    pub fn new() -> Self {
        AppState { database: database::Database::new() }
    }
}