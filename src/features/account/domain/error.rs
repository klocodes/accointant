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
}