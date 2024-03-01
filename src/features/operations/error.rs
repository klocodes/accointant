use thiserror::Error;
use crate::features::operations::domain::error::DomainError;
use crate::features::operations::infrastructure::error::InfrastructureError;

#[derive(Clone, Debug, Error)]
pub enum OperationError {
    #[error("Operation domain error. {0}")]
    Domain(DomainError),

    #[error("Operation infrastructure error. {0}")]
    Infrastructure(InfrastructureError),
}