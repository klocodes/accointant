use bcrypt;
use serde::{Deserialize, Serialize};
use crate::errors::client::ClientErrors::DomainError;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::services::hasher::Hasher;

const MIN_LENGTH: usize = 8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(hashed_value: String, raw_value: String) -> Result<Self, Error> {
        if raw_value.len() < MIN_LENGTH {
            return Err(Error::Client(
                DomainError {
                    message: "Password must be at least 8 characters long".into()
                }
            ));
        }

        Ok(Self(hashed_value))
    }


    pub fn value(&self) -> &str {
        &self.0
    }
}