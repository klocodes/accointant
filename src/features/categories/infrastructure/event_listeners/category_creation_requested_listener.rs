use async_trait::async_trait;
use crate::errors::Error;
use crate::events::event::Event;
use crate::events::event_listener::EventListener;

const EVENT_NAME: &str = "category_creation_requested";

#[derive(Clone)]
pub struct CategoryCreationRequestedListener;

impl CategoryCreationRequestedListener {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EventListener for CategoryCreationRequestedListener {
    async fn on_event(&self, event: Event) -> Result<(), Error> {
        Ok(())
    }

    fn event_name(&self) -> &str {
        EVENT_NAME
    }
}