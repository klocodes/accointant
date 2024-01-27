use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use crate::config::structs::auth::AuthConfig;
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

pub trait JwtService {
    fn create(&self, claims: Claims) -> Result<String, Error>;
    fn verify(&self, token: &str) -> Result<Claims, Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    email: String,
}

impl Claims {
    pub fn new(sub: String, exp: usize, email: String) -> Self {
        Self {
            sub,
            exp,
            email,
        }
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn exp(&self) -> &usize {
        &self.exp
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

pub struct JsonwebtokenLibService {
    cfg: AuthConfig,
}

impl JsonwebtokenLibService {
    pub fn new(cfg: AuthConfig) -> Self {
        Self {
            cfg,
        }
    }
}

impl JwtService for JsonwebtokenLibService {
    fn create(&self, claims: Claims) -> Result<String, Error> {
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.cfg.secret_key().as_ref()))
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to create token: {}", e.to_string()).into()
                        )
                    }
                )
            })
    }

    fn verify(&self, token: &str) -> Result<Claims, Error> {
        let data = decode::<Claims>(token, &DecodingKey::from_secret(self.cfg.secret_key().as_ref()), &Validation::default())
            .map_err(|e| match e.kind() {
                ErrorKind::InvalidToken => Error::Client(BadRequest { message: Some("Invalid token.".into()) }),
                ErrorKind::ExpiredSignature => Error::Client(BadRequest { message: Some("Token expired.".into()) }),
                _ => Error::Server(InternalServerError { context: Some(format!("Failed to verify jwt: {}", e.to_string()).into()) })
            })?;

        Ok(data.claims)
    }
}