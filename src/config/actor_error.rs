use thiserror::Error;

#[derive(Error, Debug)]
#[error("Actor error: {message}")]
pub struct ActorError {
    pub message: String,
}