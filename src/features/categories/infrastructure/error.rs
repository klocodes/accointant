use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InfrastructureError {
    #[error("Category repository error. {0}")]
    Repository(String)
}