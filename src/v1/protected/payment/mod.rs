use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder, dev::HttpServiceFactory,
};
use rust_serverless_backend::dynamo::{tables::USER_TABLE, self};
use serde_json;
use stripe::{CreateCustomer, Customer, ListProducts, Product};

use crate::v1::auth::middleware::Authorized;


#[get("/products")]
async fn plans(stripe: Data<stripe::Client>) -> impl Responder {
    let params = ListProducts::new();
    let products = Product::list(&stripe, &params).await.unwrap();
    let json = serde_json::to_string(&products).unwrap();
    HttpResponse::Ok().body(json)
}

#[post("/create_customer")]
async fn create_customer(
    dynamo: Data<dynamo::Client>,
    stripe: Data<stripe::Client>,
    auth_user: web::ReqData<Authorized>,
) -> impl Responder {
    let user = dynamo.get(&USER_TABLE, "id", &auth_user.id).await.unwrap();
    let mut params = CreateCustomer::new();
    params.email = Some(&user.email);
    Customer::create(&stripe, params).await.unwrap();
    HttpResponse::Ok().body("")
}

#[post("/create_subscription")]
async fn create_subscription(

) -> impl Responder {
    

    HttpResponse::Ok().body("")
}



pub fn endpoints() -> impl HttpServiceFactory {
    web::scope("/payment").service(plans)
}
