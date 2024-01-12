use actix_web::{post, HttpResponse, Responder, web::Json};
use serde::Deserialize;
use validator::Validate;
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::log_trace;

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

#[post("/register")]
async fn register(data: Json<RequestData>) -> Result<impl Responder, Error> {
    data.validate()
        .map_err(|e| Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }))?;

    Ok(HttpResponse::Ok().body("Success!"))
}