use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Email not found")]
    EmailNotFound,

    #[error("Password mismatch")]
    PasswordMismatch,

    #[error("Wrong password")]
    WrongPassword,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token has not expired yet")]
    TokenHasNotExpired,

    #[error("Email already confirmed")]
    EmailConfirmed,

    #[error("Email has not confirmed")]
    EmailHasNotConfirmed,
}