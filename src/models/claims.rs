use serde::{Deserialize, Serialize};
use jsonwebtoken::{  encode, EncodingKey, Header};
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub exp: usize,
}


impl Claims {
    pub fn new(user_id: u32) -> Self {
        let exp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize + 3600; // 1 hour
        Claims { user_id, exp }
    }
    pub fn to_token(&self) -> String {
        let header = Header::default();
        let token = encode(&header, &self, &EncodingKey::from_secret(crate::consts::SECRETKEY.as_ref())).unwrap();
        token
    }
}
