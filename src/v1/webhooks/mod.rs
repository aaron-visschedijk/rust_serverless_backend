use actix_web::{HttpResponse, Responder, web, post, dev::HttpServiceFactory};



#[post("/stripe")]
async fn webhook() -> impl Responder {
    HttpResponse::Ok().body("Stripe webhook was called but did nothing!")
}

pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/webhooks")
        .service(webhook)
}