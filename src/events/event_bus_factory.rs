use std::sync::Arc;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::events::event_listener_registry::EventListenerRegistry;
use crate::events::mq_event_bus::MqEventBus;

pub struct EventBusFactory;

impl EventBusFactory {
    pub async fn create(service_container: Arc<ServiceContainer>) -> Result<MqEventBus, Error> {
        let mut event_listener_registry = EventListenerRegistry::new(service_container.clone());
        event_listener_registry.register_listeners().await?;

        let listeners = event_listener_registry.listeners();

        let event_bus = MqEventBus::new(
            service_container.mq_manager(),
            service_container.serializer(),
            listeners,
        );

        Ok(event_bus)
    }
}
