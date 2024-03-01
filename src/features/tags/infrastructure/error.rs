use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InfrastructureError {
    #[error("Tag repository error. {0}")]
    Repository(String),
}