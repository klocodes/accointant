use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::events::error::EventError;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::events::event_channel::EventChannel;
use crate::events::event_listener::EventListener;
use crate::events::event_responder::EventResponder;
use crate::mq::connection::MqConnection;
use crate::mq::manager::MqManager;
use crate::services::serializer::Serializer;

const QUEUE_BUFFER: usize = 100;
const RESPONSE_BUFFER: usize = 100;

pub struct MqEventBus {
    broker: Arc<MqManager>,
    serializer: Serializer,
    queue: Arc<Mutex<HashMap<String, EventChannel<Event>>>>,
    responders: Arc<Mutex<HashMap<String, EventChannel<EventResponder>>>>,
}

impl MqEventBus {
    pub async fn new(broker: Arc<MqManager>, serializer: Serializer) -> Result<Self, EventError> {
        let event_bus = Self {
            broker,
            serializer,
            queue: Arc::new(
                Mutex::new(
                    HashMap::new()
                )
            ),
            responders: Arc::new(
                Mutex::new(
                    HashMap::new()
                )
            ),
        };

        Ok(
            event_bus
        )
    }

    pub async fn channel<T: Clone>(&self, channels: Arc<Mutex<HashMap<String, EventChannel<T>>>>, name: String) -> Result<EventChannel<T>, EventError> {
        let mut channels = channels.lock().await;

        if !channels.contains_key(&name) {
            channels.insert(name.clone(), EventChannel::new(QUEUE_BUFFER));
        }

        let channel = channels.get_mut(&name).ok_or(
            EventError::ChannelNotFound(name)
        )?.clone();

        Ok(channel)
    }
}

#[async_trait]
impl EventBus for MqEventBus {
    async fn publish(&mut self, event: Event) -> Result<(), EventError> {
        let mut queue_channel = self.channel::<Event>(self.queue.clone(), event.name().to_string()).await?;

        queue_channel.send(event)
            .await
            .map_err(|e|
                EventError::Publishing(
                    e.to_string()
                )
            )?;

        Ok(())
    }

    async fn subscribe(&self, mut listeners: Vec<Box<dyn EventListener>>, channel_name: String) -> Result<(), EventError> {
        let mut queue_channel = self.channel::<Event>(self.queue.clone(), channel_name.clone()).await?;
        let mut responder_channel = self.channel::<EventResponder>(self.responders.clone(), channel_name).await?;

        while let Some(event) = queue_channel.receive().await {
            for listener in listeners.iter_mut() {
                if event.name() == listener.event_name() {
                    let result = listener.on_event(event.clone()).await;

                    responder_channel.send(EventResponder::new(event.clone(), result.clone()))
                        .await
                        .map_err(|e|
                            EventError::ResponseSending(e.to_string())
                        )?;
                }
            }
        }

        Ok(())
    }
}