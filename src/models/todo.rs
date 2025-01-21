use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Todo {
    #[serde(default = "default_id")]
    pub id: u32,
    pub title: String,
    #[serde(default)]
    pub completed: bool,
}

fn default_id() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32
}

impl Todo {
    pub fn new(title: String) -> Self {
        let id = default_id();
        Todo { id, title, completed: false }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

impl From<Vec<u8>> for Todo {
    fn from(value: Vec<u8>) -> Self {
        let todo: Todo = serde_json::from_slice(&value).unwrap();
        todo
    }
}

impl From<Todo> for Vec<u8> {
    fn from(value: Todo) -> Self {
        value.to_vec()
    }
}