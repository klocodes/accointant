use uuid;
use chrono;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::auth::application::dto::user_data::UserData;
use crate::features::auth::domain::confirmation_token::{ConfirmationToken, EXPIRATION_HOURS};
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::password::Password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,

    #[serde(flatten)]
    email: Email,

    password: Password,

    #[serde(rename = "created_at")]
    registered_at: chrono::DateTime<Utc>,

    updated_at: chrono::DateTime<Utc>,

    #[serde(flatten)]
    confirmation_token: ConfirmationToken,

    confirmed_at: Option<chrono::DateTime<Utc>>,
}

impl User {
    pub fn register(data: UserData) -> Result<Self, DomainError> {
        if data.password() != data.password_confirmation() {
            return Err(DomainError::PasswordMismatch);
        }

        let email = Email::new(data.email().to_string())?;
        let password = Password::new(data.hashed_password().to_string(), data.password().to_string())?;
        let confirmation_token = ConfirmationToken::new(
            data.confirmation_token().to_string(),
            Utc::now() + Duration::hours(EXPIRATION_HOURS),
        );

        Ok(
            Self {
                id: Uuid::new_v4(),
                email,
                confirmation_token,
                password,
                registered_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                confirmed_at: None,
            }
        )
    }

    pub fn request_confirmation(&mut self, token: String) -> Result<(), DomainError> {
        if self.confirmed_at.is_some() {
            return Err(DomainError::EmailConfirmed);
        }

        if !self.confirmation_token.has_expired() {
            return Err(DomainError::TokenHasNotExpired);
        }

        self.confirmation_token = ConfirmationToken::new(
            token,
            Utc::now() + Duration::hours(EXPIRATION_HOURS),
        );
        self.updated_at = chrono::Utc::now();

        Ok(())
    }

    pub fn confirm(&mut self, token: String) -> Result<(), DomainError> {
        if token != self.confirmation_token.value() {
            return Err(DomainError::InvalidToken);
        }

        if self.confirmation_token.has_expired() {
            return Err(DomainError::TokenHasNotExpired);
        }

        if self.confirmed_at.is_some() {
            return Err(DomainError::EmailConfirmed);
        }

        self.confirmed_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();

        Ok(())
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn registered_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.registered_at
    }

    pub fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.updated_at
    }

    pub fn confirmation_token(&self) -> &ConfirmationToken {
        &self.confirmation_token
    }

    pub fn confirmed_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.confirmed_at
    }
}

#[cfg(test)]
mod register_tests {
    use super::*;
    use crate::features::auth::application::dto::user_data::UserData;

    #[test]
    fn register_user_with_valid_data() {
        let user_data = UserData::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            "password123".to_string(),
            "hashed_password".to_string(),
            "confirmation_token".to_string(),
        );

        let user = User::register(user_data).unwrap();

        assert_eq!(user.email().value(), "test@example.com");
        assert_eq!(user.password().value(), "hashed_password");
        assert!(user.confirmed_at().is_none());
    }

    #[test]
    fn register_user_fails_on_password_mismatch() {
        let user_data = UserData::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            "different_password".to_string(),
            "hashed_password".to_string(),
            "confirmation_token".to_string(),
        );

        assert!(User::register(user_data).is_err());
    }
}
