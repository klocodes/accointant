use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientErrors {
    #[error("{message:?}")]
    BadRequest { message: Option<Value> },

    #[error("{message:?}")]
    DomainError { message: Value },

    #[error("Unauthorized: The request has not been applied because it lacks valid authentication credentials for the target resource. {context:?}")]
    Unauthorized { context: Option<Value> },

    #[error("Payment Required: The client needs to authenticate to gain network access. {context:?}")]
    PaymentRequired { context: Option<Value> },

    #[error("Not Found: The origin server did not find a current representation for the target resource or is not willing to disclose that one exists. {context:?}")]
    NotFound { context: Option<Value> },
}