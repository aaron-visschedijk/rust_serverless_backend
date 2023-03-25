use serde::{Serialize, Deserialize};
use stripe::CustomerId;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub stripe_id: Option<CustomerId>,
    pub email_verified: bool,
}

impl User {
    pub fn new(email: String, password_hash: String, name: String) -> Self {
        User {
            name,
            id: Uuid::new_v4().to_string(),
            email,
            password_hash,
            stripe_id: None,
            email_verified: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RevokedToken {
    pub jwt: String,
    pub exp: usize,
}