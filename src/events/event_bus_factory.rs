use std::sync::Arc;
use tokio::sync::Mutex;
use crate::di::service_container::ServiceContainer;
use crate::events::error::EventError;
use crate::events::event_bus::EventBus;
use crate::events::event_router::EventRouter;
use crate::events::mq_event_bus::MqEventBus;

pub struct EventBusFactory;

impl EventBusFactory {
    pub async fn create(service_container: Arc<ServiceContainer>) -> Result<Arc<Mutex<Box<dyn EventBus>>>, EventError> {
        let mut event_listener_registry = EventRouter::new(service_container.clone());
        event_listener_registry.register_listeners().await?;

        let event_bus = MqEventBus::new(
            service_container.mq_manager(),
            service_container.serializer(),
        ).await?;

        Ok(
            Arc::new(
                Mutex::new(
                    Box::new(event_bus) as Box<dyn EventBus>
                )
            ),
        )
    }
}
