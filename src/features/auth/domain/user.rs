use uuid;
use chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::client::ClientErrors::{BadRequest, DomainError};
use crate::errors::Error;
use crate::features::auth::application::dto::user_data::UserData;
use crate::features::auth::domain::confirmation_token::ConfirmationToken;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,

    #[serde(flatten)]
    email: Email,
    
    password: Password,

    #[serde(rename = "created_at")]
    registered_at: chrono::DateTime<chrono::Utc>,

    updated_at: chrono::DateTime<chrono::Utc>,

    #[serde(flatten)]
    confirmation_token: ConfirmationToken,

    confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub fn register(data: UserData) -> Result<Self, Error> {
        if data.password() != data.password_confirmation() {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some("Password and password confirmation do not match".into())
                    }
                )
            );
        }

        let email = Email::new(data.email().to_string())?;
        let password = Password::new(data.password().to_string(), data.hashed_password().to_string())?;
        let confirmation_token = ConfirmationToken::new(data.confirmation_token().to_string());

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

    pub async fn request_confirmation(&mut self, token: String) -> Result<(), Error> {
        if self.confirmed_at.is_some() {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Email already confirmed".into()
                    }
                )
            );
        }

        if !self.confirmation_token.has_expired() {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Confirmation token has not expired yet".into()
                    }
                )
            );
        }

        self.confirmation_token = ConfirmationToken::new(token);
        self.updated_at = chrono::Utc::now();

        Ok(())
    }

    pub async fn confirm(&mut self, token: String) -> Result<(), Error> {
        if token != self.confirmation_token.value() {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Confirmation token is invalid".into()
                    }
                )
            );
        }

        if self.confirmation_token.has_expired() {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Confirmation token has expired".into()
                    }
                )
            );
        }

        if self.confirmed_at.is_some() {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Email already confirmed".into()
                    }
                )
            );
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
