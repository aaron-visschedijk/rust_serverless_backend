use actix_web::{Scope, Responder, get, HttpResponse, web, guard};

use super::auth::authorized;

mod user;

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("protected API is live!")
}

#[derive(Clone)]
struct Authorized(String);

impl Authorized {
    pub fn id(&self) -> &str {
        &self.0
    }
}

pub fn endpoints() -> Scope {
    web::scope("/protected")
        .guard(guard::fn_guard(|context| {
            let auth_user: Option<String> = authorized(&context);
            match auth_user {
                Some(user_id) => {
                    context.req_data_mut().insert(Authorized(user_id));
                    true
                }
                None => false,
            }
        }))
        .service(root)
        .service(user::endpoints())
}