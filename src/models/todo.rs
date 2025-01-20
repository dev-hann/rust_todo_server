use serde::{Deserialize, Serialize};
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
}
