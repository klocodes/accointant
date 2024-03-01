use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::support::error::SupportError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn generate() -> Uuid {
        Uuid::new_v4()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn from_string(id: &str) -> Result<Self, SupportError> {
    let id = Uuid::parse_str(id).map_err(|err| {
        SupportError::Id(err.to_string())
        })?;

        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
