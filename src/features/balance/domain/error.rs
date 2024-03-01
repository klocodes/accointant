use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Unknown currency: {0}")]
    UnknownCurrency(String),

    #[error("Rate cannot be negative")]
    RateCannotBeNegative,
}