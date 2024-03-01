use thiserror::Error;
use crate::features::auth::error::AuthError;
use crate::features::operations::error::OperationError;

#[derive(Debug, Clone, Error)]
pub enum FeatureError {
    #[error("Auth bounded context error. {0}")]
    Auth(AuthError),

    #[error("Operation bounded context error. {0}")]
    Operation(OperationError)
}