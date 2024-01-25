use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

const EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmationToken {
    #[serde(rename = "confirmation_token")]
    value: String,

    #[serde(rename = "confirmation_token_expires_at")]
    expires_at: DateTime<Utc>,
}

impl ConfirmationToken {
    pub fn new(value: String) -> Self {
        Self {
            value,
            expires_at: Utc::now() + Duration::hours(EXPIRATION_HOURS),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn has_expired(&self) -> bool {
        self.expires_at < chrono::Utc::now()
    }
}