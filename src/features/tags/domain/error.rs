use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Tag {0} already exists")]
    TagAlreadyExists(String),
}