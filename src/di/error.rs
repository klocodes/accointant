use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ServiceContainerError {
    #[error("DB connection error. {0}")]
    DbConnection(String),

    #[error("MQ connection error. {0}")]
    MqConnection(String),
}