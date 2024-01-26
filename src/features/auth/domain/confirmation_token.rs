use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

pub const EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationToken {
    #[serde(rename = "confirmation_token")]
    value: String,

    #[serde(rename = "confirmation_token_expires_at")]
    expires_at: DateTime<Utc>,
}

impl ConfirmationToken {
    pub fn new(value: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            value,
            expires_at,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn expires_at(&self) -> &DateTime<Utc> {
        &self.expires_at
    }

    pub fn has_expired(&self) -> bool {
        self.expires_at < chrono::Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn token_new_creates_valid_instance() {
        let value = "test_token".to_string();
        let expires_at = Utc::now() + Duration::hours(EXPIRATION_HOURS);

        let token = ConfirmationToken::new(value.clone(), expires_at);

        assert_eq!(token.value(), value.as_str());
        assert_eq!(token.expires_at(), &expires_at);
    }

    #[test]
    fn token_not_expired_when_created() {
        let token = ConfirmationToken::new("test_token".to_string(), Utc::now() + Duration::hours(EXPIRATION_HOURS));

        assert_eq!(token.has_expired(), false);
    }

    #[test]
    fn token_expired_after_expiration_time() {
        // Создаем токен с истекшим сроком действия
        let token = ConfirmationToken::new("test_token".to_string(), Utc::now() - Duration::hours(EXPIRATION_HOURS));

        assert_eq!(token.has_expired(), true);
    }
}
