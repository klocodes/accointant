use uuid;
use chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::features::auth::domain::confirmation_token::ConfirmationToken;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;
use crate::services::tokenizer::Tokenizer;

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn register(email: String, password: String, password_confirmation: String, confirmation_token: String) -> Result<Self, Error> {
        if password != password_confirmation {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some("Password and password confirmation do not match".into())
                    }
                )
            );
        }

        let email = Email::new(email)?;
        let password = Password::new(password)?;
        let confirmation_token = ConfirmationToken::new(confirmation_token);

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

    pub async fn confirm(&mut self) -> Result<(), Error> {
        self.confirmed_at = Some(chrono::Utc::now());

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

    pub fn confirmation_token(&self) -> &ConfirmationToken {
        &self.confirmation_token
    }

    pub fn confirmed_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.confirmed_at
    }
}
