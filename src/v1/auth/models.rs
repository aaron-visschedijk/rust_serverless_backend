use actix_easy_multipart::{MultipartForm, text::Text};
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct SignUp {
    pub email: String,
    pub password: String,
}

#[derive(MultipartForm)]
pub struct SignIn {
    pub email: Text<String>,
    pub password: Text<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Refresh {
    pub refresh_token: String,
    pub grant_type: String
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