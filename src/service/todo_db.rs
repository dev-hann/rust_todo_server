use crate::models::todo::Todo;
pub struct TodoDatabase {
    instance: sled::Db,
}

impl TodoDatabase {
    pub fn new() -> Self {
        let instance = match sled::open("db/todo.db") {
            Ok(instance) => instance,
            Err(e) => panic!("Failed to open database: {:?}", e),
        };
        TodoDatabase { instance }
    }

    pub fn get_todo(&self, id: u32) -> Option<Todo> {
        match self.instance.get(&id.to_string()) {
            Ok(Some(todo)) => Some(Todo::from(todo.to_vec())),
            _ => None,
        }
    }

    pub fn get_all_todos(&self) -> Vec<Todo> {
        let todos = self.instance.iter().filter_map(|res| {
            match res {
                Ok((k, v)) => Some(Todo::from(v.to_vec())),
                Err(_) => None,
            }
        }).collect::<Vec<Todo>>();
        todos
    }

    pub fn add_todo(&self, todo: Todo) {
        match self.instance.insert(todo.id.to_string(), todo.to_vec()) {
            Ok(_) => println!("Todo added successfully"),
            Err(e) => println!("Failed to add todo: {:?}", e),
        }
    }

    pub fn delete_todo(&self, id: u32) {
        match self.instance.remove(&id.to_string()) {
            Ok(_) => println!("Todo deleted successfully"),
            Err(e) => println!("Failed to delete todo: {:?}", e),
        }
    }

    pub fn update_todo(&self, todo: Todo) {
        match self.instance.insert(todo.id.to_string(), todo.to_vec()) {
            Ok(_) => println!("Todo updated successfully"),
            Err(e) => println!("Failed to update todo: {:?}", e),
        }
    }
}
