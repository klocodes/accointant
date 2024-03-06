use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Unknown currency: {0}")]
    UnknownCurrency(String),

    #[error("Unknown account source: {0}")]
    UnknownSource(String),

    #[error("Rate must be positive")]
    RateMustBePositive,

    #[error("Invalid amount. {0}")]
    InvalidAmount(String),

    #[error("Unknown operation kind: {0}")]
    UnknownOperationKind(String),

    #[error("Invalid event. {0}")]
    InvalidEvent(String),

    #[error("Account not found")]
    AccountNotFound,

    #[error("Account already exists")]
    AccountAlreadyExists,
}