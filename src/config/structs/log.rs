use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct LogConfig {
    level: String,
    rotation: String,
}

impl LogConfig {
    pub fn level(&self) -> &String {
        &self.level
    }

    pub fn rotation(&self) -> &str {
        &self.rotation
    }
}