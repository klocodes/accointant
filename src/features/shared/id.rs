use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

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

    pub fn from_string(id: &str) -> Result<Self, Error> {
    let id = Uuid::parse_str(id).map_err(|err| {
            Error::Server(
                InternalServerError {
                    context: Some(err.to_string().into()),
                }
            )
        })?;

        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
