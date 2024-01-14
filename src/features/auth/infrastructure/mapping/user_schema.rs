use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSchema {
    id: Uuid,
    email: String,
    password: String,
    registered_at: chrono::DateTime<chrono::Utc>,
    confirmation_token: Option<String>,
    confirmation_token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
}