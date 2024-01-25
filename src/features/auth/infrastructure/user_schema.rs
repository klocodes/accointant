use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::data_mapper::DataMapper;
use crate::features::auth::domain::user::User;

#[derive(Serialize, Deserialize)]

#[derive(sqlx::FromRow)]
pub struct UserSchema {
    id: Uuid,
    name: Option<String>,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    confirmation_token: Option<String>,
    confirmation_token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl DataMapper for UserSchema {
    type Schema = Self;
    type Entity = User;
}

impl UserSchema {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.updated_at
    }

    pub fn confirmation_token(&self) -> Option<&str> {
        self.confirmation_token.as_deref()
    }

    pub fn confirmation_token_expires_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.confirmation_token_expires_at.as_ref()
    }

    pub fn confirmed_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.confirmed_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.deleted_at.as_ref()
    }
}