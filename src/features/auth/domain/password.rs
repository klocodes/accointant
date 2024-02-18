use serde::{Deserialize, Serialize};
use crate::errors::client::ClientErrors::DomainError;
use crate::errors::Error;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_new() {
        let raw_password = "password123".to_string();
        let hashed_password = "hashed_password".to_string(); // Заглушка хэшированного пароля

        let password = Password::new(hashed_password.clone(), raw_password.clone());

        assert!(password.is_ok());
        assert_eq!(password.unwrap().value(), &hashed_password);
    }

    #[test]
    fn test_password_new_fail() {
        let raw_password = "short".to_string();
        let hashed_password = "hashed_password".to_string(); // Заглушка хэшированного пароля

        let password = Password::new(hashed_password.clone(), raw_password.clone());

        assert!(password.is_err());
    }

    #[test]
    fn test_password_value() {
        let raw_password = "password123".to_string();
        let hashed_password = "hashed_password".to_string(); // Заглушка хэшированного пароля

        let password = Password::new(hashed_password.clone(), raw_password.clone()).unwrap();

        assert_eq!(password.value(), &hashed_password);
    }
}
