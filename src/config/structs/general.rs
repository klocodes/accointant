use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GeneralConfig {
    env: String,
    db_connection: String,
    timezone: String,
    locale: String,
}

impl GeneralConfig {
    pub fn env(&self) -> &str {
        &self.env
    }

    pub fn database(&self) -> &str {
        &self.db_connection
    }

    pub fn timezone(&self) -> &str {
        &self.timezone
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }
}