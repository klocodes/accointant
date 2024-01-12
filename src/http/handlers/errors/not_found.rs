use actix_web::{Responder, ResponseError};
use crate::errors::client::ClientErrors;
use crate::errors::Error;

pub async fn handle() -> impl Responder {
    Error::Client(ClientErrors::NotFound {context: None}).error_response()
}
