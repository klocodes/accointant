use thiserror::Error;
use crate::features::tags::domain::error::DomainError;
use crate::features::tags::infrastructure::error::InfrastructureError;

#[derive(Error, Debug, Clone)]
pub enum TagError {
    #[error("Tag domain error: {0}")]
    Domain(DomainError),

    #[error("Tag infrastructure error: {0}")]
    Infrastructure(InfrastructureError)
}