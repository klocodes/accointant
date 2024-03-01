use thiserror::Error;
use crate::features::categories::domain::error::DomainError;
use crate::features::categories::infrastructure::error::InfrastructureError;

#[derive(Debug, Clone, Error)]
pub enum CategoryError {
    #[error("Category domain error. {0}")]
    Domain(DomainError),

    #[error("Category infrastructure error. {0}")]
    Infrastructure(InfrastructureError),
}