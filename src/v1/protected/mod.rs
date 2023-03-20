use actix_web::{Responder, get, HttpResponse, web, dev:: HttpServiceFactory};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::v1::auth::middleware::validator;

mod user;
mod payment;

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("protected API is live!")
}

pub fn endpoints() -> impl HttpServiceFactory {

    web::scope("/protected")
        .wrap(HttpAuthentication::bearer(validator))
        .service(root)
        .service(user::endpoints())
        .service(payment::endpoints())
}