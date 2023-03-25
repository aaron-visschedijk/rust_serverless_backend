use actix_web::{web, Responder, HttpResponse, get, dev::HttpServiceFactory};

pub mod auth;
mod protected;
mod webhooks;

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json("V1 API is live!")
}

pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/v1")
        
        .service(root)
        .service(auth::endpoints())
        .service(protected::endpoints())
        .service(webhooks::endpoints())
}