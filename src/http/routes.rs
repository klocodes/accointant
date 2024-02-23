use actix_web::web;
use actix_web::web::{scope, ServiceConfig};
use crate::http::handlers::account::profile;
use crate::http::handlers::auth::{confirm_registration, login, registration, request_confirmation_token};
use crate::http::handlers::{categories, operations};
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

        let operations = scope("/operations")
            .wrap(CheckAuth)
            .service(operations::create::create_operation);

        let categories = scope("/categories")
            .wrap(CheckAuth)
            .service(categories::create::create_category);


        cfg.service(auth)
            .service(account)
            .service(operations)
            .service(categories)
            .default_service(web::route().to(not_found::handle));
    }
}