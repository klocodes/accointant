use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum DomainError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Unknown currency")]
    UnknownCurrency,

    #[error("Unknown operation kind")]
    UnknownOperationKind,
}