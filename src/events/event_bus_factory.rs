use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use crate::di::service_container::ServiceContainer;
use crate::events::error::EventError;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::events::event_router::EventRouter;
use crate::events::event_responder::EventResponder;
use crate::events::mq_event_bus::MqEventBus;

pub struct EventBusFactory;

impl EventBusFactory {
    pub async fn create(service_container: Arc<ServiceContainer>) -> Result<(Arc<Box<dyn EventBus>>, Receiver<Event>, Receiver<EventResponder>), EventError> {
        let mut event_listener_registry = EventRouter::new(service_container.clone());
        event_listener_registry.register_listeners().await?;

        let listeners = event_listener_registry.listeners();

        let (event_bus, queue_receiver, responder) = MqEventBus::new(
            service_container.mq_manager(),
            service_container.serializer(),
            listeners,
        ).await?;

        Ok(
            (
                Arc::new(
                    Box::new(event_bus) as Box<dyn EventBus>
                ),
                queue_receiver,
                responder
            )
        )
    }
}
