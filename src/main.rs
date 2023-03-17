use actix_cors::Cors;
use actix_web::{http, middleware};
use lambda_web::actix_web::{self, get, web, App, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use rust_serverless_backend::dynamo;

mod v1;

#[get("/")]
async fn root() -> impl Responder {
    format!("Welcome to the Rust Serverless Backend!")
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    println!("Starting server...");
    let aws_config = aws_config::load_from_env().await;
    let factory = move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(dynamo::Client::new(&aws_config)))
            .service(root)
            .service(v1::endpoints())
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind("127.0.0.1:8000")?
            .run()
            .await?;
    }
    Ok(())
}
