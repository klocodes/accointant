use async_trait::async_trait;
use crate::events::error::EventError;
use crate::events::event::Event;
use crate::events::event_listener::EventListener;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {
    async fn publish(&mut self, event: Event) -> Result<(), EventError>;

    async fn subscribe(&self, listeners: Vec<Box<dyn EventListener>>, channel_name: String) -> Result<(), EventError>;
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
    async fn publish(&mut self, _event: Event) -> Result<(), EventError> {
        if self.has_error {
            return Err(
                EventError::Publishing(
                    "Error publishing event".to_string()
                )
            );
        }

        Ok(())
    }

    async fn subscribe(&self, _listeners: Vec<Box<dyn EventListener>>, _channel_name: String) -> Result<(), EventError> {
        if self.has_error {
            return Err(
                EventError::Subscribing(
                    "Error subscribing to channel".to_string()
                )
            );
        }

        Ok(())
    }
}