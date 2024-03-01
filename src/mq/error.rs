use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum MqError {
    #[error("Failed to send message to MQ. {0}")]
    Sending(String),

    #[error("Failed to consume message from MQ. {0}")]
    Consuming(String),

    #[error("Failed to connect to MQ. {0}")]
    Connection(String),

    #[error("MQ channel error. {0}")]
    Channel(String),

    #[error("MQ queue error. {0}")]
    Queue(String),
}