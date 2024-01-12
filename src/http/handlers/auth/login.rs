use actix_web::{post, Responder};
use actix_web::http::Error;
use actix_web::web::{Form};
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestData {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(data: Form<RequestData>) -> Result<impl Responder, Error> {

    Ok(format!("Hello {}! Your password is '{}'", data.email, data.password))
}