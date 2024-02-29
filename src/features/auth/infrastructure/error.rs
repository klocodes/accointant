use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum InfrastructureError {
    #[error("Repository error. {0}")]
    Repository(String),

    #[error("Jwt error. {0}")]
    Jwt(String),

    #[error("Mailer error. {0}")]
    Mailer(String),

    #[error("Templater error. {0}")]
    Templater(String),

    #[error("Tokenizer error. {0}")]
    Tokenizer(String),

    #[error("Hasher error. {0}")]
    Hasher(String),

    #[error("Transaction error. {0}")]
    Transaction(String),
}