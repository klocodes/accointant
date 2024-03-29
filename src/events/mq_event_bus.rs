use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::events::error::EventError;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::events::event_listener::EventListener;
use crate::events::event_responder::EventResponder;
use crate::mq::connection::MqConnection;
use crate::mq::manager::MqManager;
use crate::mq::message::Message;
use crate::services::serializer::Serializer;

const QUEUE_BUFFER: usize = 100;
const RESPONSE_BUFFER: usize = 100;

pub struct MqEventBus {
    broker: Arc<MqManager>,
    serializer: Serializer,
    listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>,
    queue: Arc<Mutex<Sender<Event>>>,
    responder: Arc<Mutex<Sender<EventResponder>>>,
}

impl MqEventBus {
    pub async fn new(broker: Arc<MqManager>, serializer: Serializer, listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>) -> Result<(Self, Receiver<Event>, Receiver<EventResponder>), EventError> {
        let (queue_sender, queue_receiver) = tokio::sync::mpsc::channel(QUEUE_BUFFER);
        let (responder, response) = tokio::sync::mpsc::channel(RESPONSE_BUFFER);

        let event_bus = Self {
            broker,
            serializer,
            listeners,
            queue: Arc::new(Mutex::new(queue_sender)),
            responder: Arc::new(Mutex::new(responder)),
        };

        Ok(
            (event_bus, queue_receiver, response)
        )
    }
}

#[async_trait]
impl EventBus for MqEventBus {
    async fn publish(&self, event: Event) -> Result<(), EventError> {
        let mut guard = self.queue.lock().await;

        guard.send(event)
            .await
            .map_err(|e|
            EventError::Publishing(
                e.to_string()
            )
        )?;

        Ok(())
    }

    async fn start(&self, mut receiver: Receiver<Event>) -> Result<(), EventError> {
        while let Some(event) = receiver.recv().await {
            let data = self.serializer.serialize(&event)
                .map_err(|e|
                    EventError::Service(e.to_string())
                )?;

            let message = Message::new(data);

            self.broker.connection().send(message).await
                .map_err(|e|
                    EventError::Service(e.to_string())
                )?;

            let mut listeners = self.listeners.lock().await;

            for listener in listeners.iter_mut() {
                if event.name() == listener.event_name() {
                    let result = listener.on_event(event.clone()).await;
                    let guard = self.responder.lock().await;
                    guard.send(EventResponder::new(event.clone(), result.clone())).await.map_err(|e|
                        EventError::ResponseSending(e.to_string())
                    )?;

                    if let Ok(events) = result {
                        for event in events {
                            let guard = self.queue.lock().await;

                            guard.send(event)
                                .await
                                .map_err(|e|
                                   EventError::QueueSending(e.to_string())
                                )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}