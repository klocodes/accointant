use actix_web::{get, HttpResponse, Responder};
use crate::errors::Error;

#[get("/profile")]
async fn profile() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json("profile"))
}