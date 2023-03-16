use actix_web::{Scope, Responder, get, HttpResponse, web};
use rust_serverless_backend::dynamo::{Client, tables::USER_TABLE};
use crate::v1::protected::Authorized;

mod models;

#[get("/me")]
async fn me(client: web::Data<Client>, auth_user: web::ReqData<Authorized>) -> impl Responder {
    let user = client.get(&USER_TABLE, "user_id", auth_user.id()).await;
    match user {
        Some(user) => HttpResponse::Ok().json(models::MeResponse {user_id: user.user_id, email: user.email}),
        None => HttpResponse::InternalServerError().body("User not found!")
    }
}

pub fn endpoints() -> Scope {
    web::scope("/user")
        .service(me)
}