use uuid;
use chrono;

use crate::features::auth::domain::token::Token;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;

#[derive(Debug)]
pub struct User {
    id: uuid::Uuid,
    email: Email,
    password: Password,
    registered_at: chrono::DateTime<chrono::Utc>,
    confirmation_token: Option<Token>,
}

impl User {
    pub fn new(email: Email, password: Password, password_confirmation: Password) -> Self {
        if password.value() != password_confirmation.value() {
            panic!("Passwords do not match");
        }

        Self {
            id: uuid::Uuid::new_v4(),
            email,
            confirmation_token: None,
            password,
            registered_at: chrono::Utc::now(),
        }
    }

    pub fn request_confirmation(&mut self) {
        if self.confirmation_token.is_some() && !self.confirmation_token.as_ref().unwrap().is_expired() {
            return;
        }

        self.confirmation_token = Some(Token::new())
    }

    pub fn id(&self) -> &uuid::Uuid {
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

    pub fn confirmation_token(&self) -> Option<&Token> {
        self.confirmation_token.as_ref()
    }
}
