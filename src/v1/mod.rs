use actix_web::{web, Scope, Responder, HttpResponse, get};

mod auth;
mod protected;

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("V1 API is live!")
}

pub fn endpoints() -> Scope {
    web::scope("/v1")
        .service(root)
        .service(auth::endpoints())
        .service(protected::endpoints())
}