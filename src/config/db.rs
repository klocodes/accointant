use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pg: PostgresConfig,
}

#[derive(Deserialize, Clone)]
pub struct PostgresConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    name: String,
}

impl DatabaseConfig {
    pub fn pg(&self) -> &PostgresConfig {
        &self.pg
    }
}

impl PostgresConfig {
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
}