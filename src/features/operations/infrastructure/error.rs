use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InfrastructureError {
    #[error("Repository error. {0}")]
    Repository(String),
}