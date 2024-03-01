use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DomainError {
    #[error("Category {0} already exists")]
    CategoryAlreadyExists(String),

    #[error("Category {0} not found")]
    CategoryNotFound(String),
}