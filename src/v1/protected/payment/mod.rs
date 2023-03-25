use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use rust_serverless_backend::{
    dynamo::{self, tables::USER_TABLE},
    stripe_utils,
};
use serde_json;
use stripe::{ListProducts, Price, Product};

use crate::v1::auth::middleware::Authorized;

mod models;

#[get("/products")]
async fn plans(stripe: Data<stripe::Client>) -> impl Responder {
    let params = ListProducts::new();
    let products = Product::list(&stripe, &params).await.unwrap();
    let json = serde_json::to_string(&products).unwrap();
    HttpResponse::Ok().json(json)
}

#[post("/create_customer")]
async fn create_customer(
    dynamo: Data<dynamo::Client>,
    stripe: Data<stripe::Client>,
    auth_user: web::ReqData<Authorized>,
) -> impl Responder {
    let user = dynamo.get(&USER_TABLE, "id", &auth_user.id).await.unwrap();
    let customer_id = stripe_utils::create_customer(&stripe, &user).await.unwrap();
    dynamo
        .update(&USER_TABLE, &auth_user.id, "stripe_id", &customer_id)
        .await;
    HttpResponse::Ok().json(format!("customer_id: {}", customer_id))
}

#[post("/create_subscription")]
async fn create_subscription(
    dynamo: Data<dynamo::Client>,
    stripe: Data<stripe::Client>,
    auth_user: web::ReqData<Authorized>,
    price: web::Json<Price>,
) -> impl Responder {
    let user = dynamo.get(&USER_TABLE, "id", &auth_user.id).await.unwrap();
    let subscription = stripe_utils::create_subscription(&stripe, &user, &price.id)
        .await
        .unwrap();

    HttpResponse::Ok().json(models::SubscriptionCreated {
        id: subscription.id.to_string(),
        client_secret: subscription
            .latest_invoice
            .unwrap()
            .into_object()
            .unwrap()
            .payment_intent
            .unwrap()
            .into_object()
            .unwrap()
            .client_secret
            .unwrap()
    })
}

#[post("/add_payment_method")]
async fn add_payment_method() -> impl Responder {
    HttpResponse::Ok().json("add_payment_method")
}

pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/payment")
        .service(plans)
        .service(create_customer)
        .service(create_subscription)
        .service(add_payment_method)
}
