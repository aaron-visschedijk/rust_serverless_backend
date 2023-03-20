use actix_web::{Responder, get, HttpResponse, web::{self, ReqData}, dev::HttpServiceFactory};
use rust_serverless_backend::dynamo::{Client, tables::USER_TABLE};

use crate::v1::auth::middleware::Authorized;

mod models;


#[get("/me")]
async fn me(client: web::Data<Client>, auth_user: ReqData<Authorized>) -> impl Responder {
    let user = client.get(&USER_TABLE, "user_id", &auth_user.id).await;
    match user {
        Some(user) => HttpResponse::Ok().json(models::MeResponse {user_id: user.user_id, email: user.email}),
        None => HttpResponse::InternalServerError().body("User not found!")
    }
}

pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/user")
        .service(me)
}