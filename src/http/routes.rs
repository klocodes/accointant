use actix_web::web;
use actix_web::web::{scope, ServiceConfig};
use crate::http::handlers::auth::{confirm_registration, login, registration, request_confirmation_token};
use crate::http::handlers::errors::not_found;

pub struct Routes;

impl Routes {
    pub fn new(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/auth")
                .service(registration::register)
                .service(confirm_registration::confirm)
                .service(request_confirmation_token::request)
                .service(login::login)
        ).default_service(web::route().to(not_found::handle));
    }
}