use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User {
            user_id: Uuid::new_v4().to_string(),
            email: email,
            password_hash: password_hash,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RevokedToken {
    pub token: String,
    pub exp: usize,
}