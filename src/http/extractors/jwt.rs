use actix_web::{Error, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use futures_util::future::{ready, Ready};

pub struct Jwt(pub String);

impl FromRequest for Jwt {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(header_value) = auth_header.to_str() {
                let token = header_value.trim_start_matches("Bearer ");
                return ready(Ok(Jwt(token.to_string())));
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("No Authorization header")))
    }
}
