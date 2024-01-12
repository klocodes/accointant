use actix_web::web::ServiceConfig;

pub mod login;
pub mod registration;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login::login);
    cfg.service(registration::register);
}