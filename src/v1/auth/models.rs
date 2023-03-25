use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Verify {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Refresh {
    pub refresh_token: String,
    pub grant_type: String
}

#[derive(Deserialize, Serialize)]
pub struct Logout {
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignInResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignOutResponse {
    pub access_token: String,
    pub refresh_token: String,
}