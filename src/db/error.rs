use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DbError {
    #[error("Db connection error. {0}")]
    Connection(String),

    #[error("Db transaction error. {0}")]
    Transaction(String),

    #[error("Mock database manager does not have a pool")]
    MockPool,

    #[error("Mock database manager has an error")]
    Mock,
}