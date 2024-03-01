use serde::{Deserialize, Serialize};
use crate::features::operations::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Credit,
    Debt,
    Expense,
    Income,
    Transfer,
}

impl Kind {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        match value {
            "Credit" => Ok(Self::Credit),
            "Debt" => Ok(Self::Debt),
            "Expense" => Ok(Self::Expense),
            "Income" => Ok(Self::Income),
            "Transfer" => Ok(Self::Transfer),
            _ => Err(DomainError::UnknownOperationKind),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Credit => "Credit",
            Self::Debt => "Debt",
            Self::Expense => "Expense",
            Self::Income => "Income",
            Self::Transfer => "Transfer",
        }
    }
}