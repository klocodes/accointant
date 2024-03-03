use thiserror::Error;
use crate::features::account::domain::error::DomainError;
use crate::features::account::infrastructure::error::InfrastructureError;

#[derive(Error, Debug, Clone)]
pub enum AccountError {
    #[error("Account domain error. {0}")]
    Domain(DomainError),

    #[error("Account infrastructure error. {0}")]
    Infrastructure(InfrastructureError),
}