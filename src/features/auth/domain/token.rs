#[derive(Debug)]
pub struct Token {
    value: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl Token {
    pub fn new() -> Self {
        Self {
            value: uuid::Uuid::new_v4().to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < chrono::Utc::now()
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn expires_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.expires_at
    }
}