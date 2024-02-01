use async_trait::async_trait;
use crate::errors::Error;
use crate::events::event::Event;

#[async_trait]
pub trait EventListener: Send + Sync + 'static
{
    async fn on_event(&self, event: Event) -> Result<(), Error>;

    fn event_name(&self) -> &str;
}