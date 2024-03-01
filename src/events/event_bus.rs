use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use crate::events::error::EventError;
use crate::events::event::Event;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {
    async fn publish(&self, event: Event) -> Result<(), EventError>;

    async fn start(&self, receiver: Receiver<Event>) -> Result<(), EventError>;
}

pub struct MockEventBus {
    has_error: bool,
}

impl MockEventBus {
    pub fn new(has_error: bool) -> Self {
        Self {
            has_error
        }
    }
}

#[async_trait]
impl EventBus for MockEventBus {
    async fn publish(&self, _event: Event) -> Result<(), EventError> {
        if self.has_error {
            return Err(
                EventError::Publishing(
                    "Error publishing event".to_string()
                )
            );
        }

        Ok(())
    }

    async fn start(&self, _event_receiver: Receiver<Event>) -> Result<(), EventError> {
        Ok(())
    }
}