use thiserror::Error;
use crate::features::balance::domain::error::DomainError;
use crate::features::balance::infrastructure::error::InfrastructureError;

#[derive(Error, Debug, Clone)]
pub enum BalanceError {
    #[error("Balance domain error. {0}")]
    Domain(DomainError),

    #[error("Balance infrastructure error. {0}")]
    Infrastructure(InfrastructureError),
}