use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::events::event::Event;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {
    async fn publish(&self, event: Event) -> Result<(), Error>;

    async fn start(&self, receiver: Receiver<Event>) -> Result<(), Error>;
}

#[cfg(test)]
pub struct MockEventBus {
    has_error: bool,
}

#[cfg(test)]
impl MockEventBus {
    pub fn new(has_error: bool) -> Self {
        Self {
            has_error
        }
    }
}

#[cfg(test)]
#[async_trait]
impl EventBus for MockEventBus {
    async fn publish(&self, _event: Event) -> Result<(), Error> {
        if self.has_error {
            return Err(Error::Server(InternalServerError {
                context: Some("Error".into())
            }));
        }

        Ok(())
    }

    async fn start(&self, _event_receiver: Receiver<Event>) -> Result<(), Error> {
        Ok(())
    }
}