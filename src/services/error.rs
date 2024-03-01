use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ServiceError {
    #[error("Hasher service error. {0}")]
    Hasher(String),

    #[error("Http client error. {0}")]
    HttpClient(String),

    #[error("Jwt service error. {0}")]
    Jwt(String),

    #[error("Mailer service error. {0}")]
    Mailer(String),

    #[error("Serializer service error. {0}")]
    Serializer(String),

    #[error("Templater service error. {0}")]
    Templater(String),

    #[error("Tokenizer service error. {0}")]
    Tokenizer(String),
}