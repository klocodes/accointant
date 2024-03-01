use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum EventError {
    #[error("Event publishing error. {0}")]
    Publishing(String)
}