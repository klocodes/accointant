use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::events::event_listener::EventListener;
use crate::mq::connection::MqConnection;
use crate::mq::manager::MqManager;
use crate::mq::message::Message;
use crate::services::serializer::Serializer;

pub struct MqEventBus {
    broker: Arc<MqManager>,
    serializer: Serializer,
    listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>,
}

impl MqEventBus {
    pub fn new(broker: Arc<MqManager>, serializer: Serializer, listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>) -> Self {
        Self {
            broker,
            serializer,
            listeners,
        }
    }
}

#[async_trait]
impl EventBus for MqEventBus {
    async fn publish(&mut self, event: Event) -> Result<(), crate::errors::Error> {
        let data = self.serializer.serialize(&event)?;
        let message = Message::new(data);

        self.broker.connection().send(message).await?;

        let mut listeners = self.listeners.lock().await;

        for listener in listeners.iter_mut() {
           if event.name() == listener.event_name() {
               listener.on_event(event.clone()).await?;
           }
        }

        Ok(())
    }
}