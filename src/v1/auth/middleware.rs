use core::fmt;

use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use serde::Serialize;

use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::jwt;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone)]
pub struct Authorized{pub id: String}


pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let auth_user = jwt::verify_access_token(token);
    if auth_user.is_err() {
        print!("Error: {:#?}", auth_user.err().unwrap());
        return Err((ErrorUnauthorized(ErrorResponse {
            status: "error".to_string(),
            message: "Invalid token".to_string(),
        }), req));
    }
    let user_id = auth_user.unwrap().sub;
    req.extensions_mut().insert(Authorized{id: user_id});

    Ok(req)
}