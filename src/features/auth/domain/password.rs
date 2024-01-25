use bcrypt;
use serde::{Deserialize, Serialize};
use crate::errors::client::ClientErrors::DomainError;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

const MIN_LENGTH: usize = 8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(value: String) -> Result<Self, Error> {
        if value.len() < MIN_LENGTH {
            return Err(Error::Client(
                DomainError {
                    message: "Password must be at least 8 characters long".into()
                }
            ));
        }

        let hashed = bcrypt::hash(value, bcrypt::DEFAULT_COST)
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to hash password: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        Ok(Self(hashed))
    }


    pub fn value(&self) -> &str {
        &self.0
    }
}