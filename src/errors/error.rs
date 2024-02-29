use thiserror::Error;
use crate::features::auth::error::AuthError;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("Auth error. {0}")]
    Auth(AuthError),

    #[error("Request validation error. {0}")]
    RequestValidation(String),

    #[error("Service error. {0}")]
    Service(String),
}