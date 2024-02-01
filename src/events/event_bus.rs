use async_trait::async_trait;
use crate::errors::Error;
use crate::events::event::Event;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {

    async fn publish(&mut self, event: Event) -> Result<(), Error>;
}