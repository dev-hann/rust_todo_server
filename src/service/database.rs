use crate::models::todo::Todo;
pub struct Database {
    instance: sled::Db,
}

impl Database {
    pub fn new() -> Self {
        let instance = sled::open("todo.db").unwrap();
        Database { instance }
    }

    pub fn get_all_todos(&self) -> Vec<Todo> {
        let todos = self.instance.get("todos").unwrap();
        todos.iter().map(|todo| todo.to_vec()).collect()
    }

    pub fn add_todo(&self, todo: Todo) {
        self.instance.insert(todo.id.to_string(), todo.to_vec());
    }
}
