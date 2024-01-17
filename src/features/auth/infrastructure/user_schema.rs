use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::db::data_mapper::DataMapper;
use crate::features::auth::domain::user::User;

#[derive(Serialize, Deserialize, Debug, FromRow)]
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