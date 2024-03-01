use thiserror::Error;
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::infrastructure::error::InfrastructureError;

#[derive(Clone, Debug, Error)]
pub enum AuthError {
    #[error("Domain error. {0}")]
    Domain(DomainError),

    #[error("Infrastructure error. {0}")]
    Infrastructure(InfrastructureError)
}