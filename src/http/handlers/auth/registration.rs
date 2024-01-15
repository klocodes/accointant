use actix_web::{post, HttpResponse, Responder, web::Json};
use serde::Deserialize;
use validator::Validate;

use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::auth::application::command::register::register;

#[derive(Deserialize, Validate)]
pub struct RequestData {
    #[validate(email)]
    email: String,

    #[validate(length(min = 6))]
    password: String,

    #[validate(must_match = "password")]
    password_confirmation: String,
}

impl RequestData {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn password_confirmation(&self) -> &str {
        &self.password_confirmation
    }
}

#[post("/signup")]
async fn signup(data: Json<RequestData>) -> Result<impl Responder, Error> {
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

    register(data.into_inner())
        .map_err(|e| Error::Server(InternalServerError {context: Some(e.to_string().into()) }))?;

    Ok(HttpResponse::Ok())
}