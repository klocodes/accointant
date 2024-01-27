use actix_web::web;
use actix_web::web::{scope, service, ServiceConfig};
use crate::http::handlers::account::profile;
use crate::http::handlers::auth::{confirm_registration, login, registration, request_confirmation_token};
use crate::http::handlers::errors::not_found;
use crate::http::middleware::check_auth::CheckAuth;

pub struct Routes;

impl Routes {
    pub fn new(cfg: &mut ServiceConfig) {
        let auth = scope("/auth")
            .service(registration::register)
            .service(confirm_registration::confirm)
            .service(request_confirmation_token::request)
            .service(login::login);


        let account = scope("/account")
            .wrap(CheckAuth)
            .service(profile::profile);


        cfg.service(auth)
            .service(account)
            .default_service(web::route().to(not_found::handle));
    }
}