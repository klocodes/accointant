use serde::{Deserialize, Serialize};
use crate::features::account::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationKind {
    Income,
    Expense,
}

impl OperationKind {
    pub fn new(kind: &str) -> Result<Self, DomainError> {
        match kind {
            "Income" => Ok(Self::Income),
            "Expense" => Ok(Self::Expense),
            _ => Err(
                DomainError::UnknownOperationKind(kind.to_string())
            ),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Income => "Income",
            Self::Expense => "Expense",
        }
    }
}