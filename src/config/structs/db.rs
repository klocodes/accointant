use std::time::Duration;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DbConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    name: String,
    driver: String,
    max_connections: u32,
    timeout: u64,
}

impl DbConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn driver(&self) -> &str {
        &self.driver
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }
}