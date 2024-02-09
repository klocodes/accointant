use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MqConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    driver: String,
}

impl MqConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn driver(&self) -> &str {
        &self.driver
    }
}