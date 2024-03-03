use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InfrastructureError {
    #[error("Account repository error. {0}")]
    Repository(String),

    #[error("Transaction error. {0}")]
    Transaction(String),
}