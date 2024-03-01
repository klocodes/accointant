use actix_web::{Responder, ResponseError};
use crate::http::error::HttpError;

pub async fn handle() -> impl Responder {
    HttpError::NotFound.error_response()
}
