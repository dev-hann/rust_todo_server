use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct User {
    #[serde(default = "default_id")]
    pub id: u32,
    pub password: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
}

fn default_id() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

impl User {
    pub fn new(name: String, password: String, email: Option<String>) -> Self {
        User { id: default_id(), name, password, email }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn get_user_data(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl From<Vec<u8>> for User {
    fn from(value: Vec<u8>) -> Self {
        let user: User = serde_json::from_slice(&value).unwrap();
        user
    }
}

impl Into<Vec<u8>> for User {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}