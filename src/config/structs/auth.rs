use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct AuthConfig {
    secret_key: String,
}

impl AuthConfig {
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }
}
