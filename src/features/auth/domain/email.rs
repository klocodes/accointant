use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::features::auth::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    #[serde(rename = "email")]
    value: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        let email = Self {
            value: email
        };

        if let Err(e) = email.validate() {
            return Err(
                DomainError::InvalidEmail(e.to_string())
            );
        }

        Ok(email)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let valid_email = String::from("test@example.com");
        let email = Email::new(valid_email.clone()).unwrap();
        assert_eq!(email.value(), valid_email.as_str());
    }

    #[test]
    fn test_invalid_email() {
        let invalid_email = String::from("invalid_email");
        assert!(Email::new(invalid_email).is_err());
    }
}
