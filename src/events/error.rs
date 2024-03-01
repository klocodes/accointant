use thiserror::Error;
use crate::support::error::FeatureError;

#[derive(Clone, Debug, Error)]
pub enum EventError {
    #[error("Event publishing error. {0}")]
    Publishing(String),

    #[error("Feature handling event error. {0}")]
    Feature(FeatureError),

    #[error("Event parsing error. {0}")]
    Parsing(String),

    #[error("Event response sending error. {0}")]
    ResponseSending(String),

    #[error("Event sending to queue error. {0}")]
    QueueSending(String),

    #[error("Event service error. {0}")]
    Service(String)
}