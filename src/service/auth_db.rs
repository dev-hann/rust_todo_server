use crate::models::user::User;
use std::error::Error;
pub struct AuthDatabase {
    instance: sled::Db,
}

impl AuthDatabase {
    pub fn new() -> Self {
        let instance = match sled::open("db/auth.db") {
            Ok(instance) => instance,
            Err(e) => panic!("Failed to open database: {:?}", e),
        };
        AuthDatabase { instance }
    }

    pub fn get_user(&self, id: u32) -> Option<User> {
        match self.instance.get(&id.to_string()) {
            Ok(Some(user)) => Some(User::from(user.to_vec())),
            _ => None,
        }
    }

    pub fn create_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        self.instance.insert(&user.get_id().to_string(), user.get_user_data())?;
        Ok(())
    }

    pub fn update_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        self.instance.insert(&user.get_id().to_string(), user.get_user_data())?;
        Ok(())
    }

    pub fn delete_user(&self, id: u32) -> Result<(), Box<dyn Error>> {
        self.instance.remove(&id.to_string())?;
        Ok(())
    }

    pub fn get_user_by_name(&self, name: String) -> Option<User> {
        let users = self.get_all_users();
        users.iter().find(|user| user.get_name() == name).cloned()
    }

    pub fn get_all_users(&self) -> Vec<User> {
        let users = self.instance.iter().filter_map(|res| {
            match res {
                Ok((k, v)) => Some(User::from(v.to_vec())),
                Err(_) => None,
            }
        }).collect::<Vec<User>>();
        users
    }
}   
