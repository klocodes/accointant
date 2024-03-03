use serde::{Deserialize, Serialize};
use crate::features::account::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Kaspi,
    TBC,
    PayPal,
}

impl Source {
    pub fn new(source: String) -> Result<Self, DomainError> {
        match source.as_str() {
            "Kaspi" => Ok(Self::Kaspi),
            "TBC" => Ok(Self::TBC),
            "PayPal" => Ok(Self::PayPal),
            _ => Err(
                DomainError::UnknownSource(source.to_string())
            ),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Kaspi => "Kaspi",
            Self::TBC => "TBC",
            Self::PayPal => "PayPal",
        }
    }
}