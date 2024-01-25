use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::errors::client::ClientErrors::{DomainError};
use crate::errors::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    #[serde(rename = "email")]
    value: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, Error> {
        let email = Self {
            value: email
        };

        if let Err(e) = email.validate() {
            return Err(
                Error::Client(
                    DomainError {
                        message: e.to_string().into()
                    }
                )
            );
        }

        Ok(email)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}