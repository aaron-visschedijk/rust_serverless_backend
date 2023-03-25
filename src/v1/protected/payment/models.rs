use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionCreated {
    pub id: String,
    pub client_secret: String,
}