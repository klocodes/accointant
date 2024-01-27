use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TemplaterConfig {
    dir: String,
}

impl TemplaterConfig {
    pub fn dir(&self) -> &str {
        &self.dir
    }
}
