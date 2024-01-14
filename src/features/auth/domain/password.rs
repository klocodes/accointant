use bcrypt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn new(value: &str) -> Result<Self, &'static str> {
        if value.len() >= 8 {
            Ok(
                Self(
                    bcrypt::hash(value, bcrypt::DEFAULT_COST)
                        .map_err(|_| "Failed to hash password")?
                        .to_string()
                )
            )
        } else {
            Err("Password too short")
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}