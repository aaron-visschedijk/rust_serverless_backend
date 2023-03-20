mod jwt;
mod models;
mod password;
pub mod middleware;

use actix_easy_multipart::MultipartForm;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Form;
use actix_web::{web, Responder, HttpResponse, post};
use rust_serverless_backend::dynamo::{Client, models::{User, RevokedToken}};
use rust_serverless_backend::dynamo::tables::{USER_TABLE, REVOKED_TOKENS};


#[post("/register")]
async fn register(dynamo_client: web::Data<Client>, data: web::Json<models::SignUp>) -> impl Responder {
    if dynamo_client.exists(&USER_TABLE, "email", &data.email).await {
        return HttpResponse::BadRequest().body("Email already exists!");
    }
    let user = User::new(data.email.to_string(), password::hash(&data.password));
    dynamo_client.put(&USER_TABLE, &user).await;
    HttpResponse::Ok().body(format!("Succesfully created user: {:?}", user))
}

#[post("/login")]
async fn login(dynamo_client: web::Data<Client>, data: MultipartForm<models::SignIn>) -> impl Responder {
    let user = dynamo_client.get(&USER_TABLE, "email", &data.email).await;
    match user {
        Some(user) if password::verify(&data.password, &user.password_hash) => {
            let access_token = jwt::create_access_token(&user.user_id).unwrap();
            let refresh_token = jwt::create_refresh_token(&user.user_id).unwrap();
            HttpResponse::Ok().json(models::SignInResponse {access_token, refresh_token})
        }
        _ => HttpResponse::BadRequest().body("Incorrect password or user does not exist!")
    }
}

#[post("/refresh")]
async fn refresh(dynamo_client: web::Data<Client>, Form(refresh): Form<models::Refresh>) -> impl Responder {
    if refresh.grant_type != "refresh_token" {
        return HttpResponse::BadRequest().body("Invalid grant type!");
    }
    let claims = jwt::verify_refresh_token(&refresh.refresh_token).ok();
    if claims.is_none() {
        return HttpResponse::BadRequest().body("Invalid refresh token!");
    }
    let claims = claims.unwrap();
    if dynamo_client.exists(&REVOKED_TOKENS, "user_id", &claims.sub).await {
        return HttpResponse::BadRequest().body("Refresh token has been revoked!");
    }
    let access_token = jwt::create_access_token(&claims.sub).unwrap();
    let refresh_token = jwt::create_refresh_token(&claims.sub).unwrap();
    let to_revoke = RevokedToken {token: refresh.refresh_token, exp: claims.exp};
    dynamo_client.put(&REVOKED_TOKENS, &to_revoke).await;
    HttpResponse::Ok().json(models::RefreshResponse {access_token, refresh_token})
}

#[post("/logout")]
async fn logout(dynamo_client: web::Data<Client>, Form(refresh_token): Form<models::Refresh>) -> impl Responder {
    let claims = jwt::verify_refresh_token(&refresh_token.refresh_token).ok();
    match claims {
        Some(claims) => {
            let to_revoke = RevokedToken {token: refresh_token.refresh_token, exp: claims.exp};
            dynamo_client.put(&REVOKED_TOKENS, &to_revoke).await;
            HttpResponse::Ok().json(models::SignOutResponse {access_token: "".to_string(), refresh_token: "".to_string()})
        }
        None => HttpResponse::BadRequest().body("Invalid refresh token, cannot revoke!")
    }
}

pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/auth")
        .service(register)
        .service(login)
        .service(refresh)
        .service(logout)
}