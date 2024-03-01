use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InfrastructureError {
    #[error("Balance repository error: {0}")]
    Repository(String)
}