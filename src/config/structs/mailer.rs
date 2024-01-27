use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct MailerConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    from: String,
}

impl MailerConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn from(&self) -> &str {
        &self.from
    }
}