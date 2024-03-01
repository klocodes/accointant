use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::config::structs::auth::AuthConfig;
use crate::services::error::ServiceError;

pub trait JwtService {
    fn create(&self, claims: Claims) -> Result<String, ServiceError>;
    fn verify(&self, token: &str) -> Result<Claims, ServiceError>;
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

    pub fn user_id(&self) -> Result<Uuid, ServiceError> {
        Uuid::parse_str(&self.sub()).map_err(|e| {
            ServiceError::Jwt(e.to_string())
        })
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
    fn create(&self, claims: Claims) -> Result<String, ServiceError> {
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.cfg.secret_key().as_ref()))
            .map_err(|e| {
                ServiceError::Jwt(e.to_string())
            })
    }

    fn verify(&self, token: &str) -> Result<Claims, ServiceError> {
        let data = decode::<Claims>(token, &DecodingKey::from_secret(self.cfg.secret_key().as_ref()), &Validation::default())
            .map_err(|e| match e.kind() {
                ErrorKind::InvalidToken => ServiceError::Jwt("Invalid token.".into()),
                ErrorKind::ExpiredSignature => ServiceError::Jwt("Token expired.".into()),
                _ => ServiceError::Jwt(e.to_string()),
            })?;

        Ok(data.claims)
    }
}