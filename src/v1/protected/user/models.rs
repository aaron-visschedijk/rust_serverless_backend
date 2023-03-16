use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct MeResponse {
    pub user_id: String,
    pub email: String,
}