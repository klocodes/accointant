use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::events::event_listener::EventListener;
use crate::mq::connection::MqConnection;
use crate::mq::manager::MqManager;
use crate::mq::message::Message;
use crate::services::serializer::Serializer;

const QUEUE_BUFFER: usize = 100;

pub struct MqEventBus {
    broker: Arc<MqManager>,
    serializer: Serializer,
    listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>,
    queue: Arc<Mutex<Sender<Event>>>,
}

impl MqEventBus {
    pub async fn new(broker: Arc<MqManager>, serializer: Serializer, listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>) -> Result<(Self, Receiver<Event>), Error> {
        let (sender, receiver) = tokio::sync::mpsc::channel(QUEUE_BUFFER);

        let event_bus = Self {
            broker,
            serializer,
            listeners,
            queue: Arc::new(Mutex::new(sender)),
        };

        Ok(
            (event_bus, receiver)
        )
    }
}

#[async_trait]
impl EventBus for MqEventBus {
    async fn publish(&self, event: Event) -> Result<(), Error> {
        let mut guard = self.queue.lock().await;

        guard.send(event)
            .await.map_err(|e|
            Error::Server(
                InternalServerError {
                    context: Some(e.to_string().into())
                }
            )
        )?;

        Ok(())
    }

    async fn start(&self, mut receiver: Receiver<Event>) -> Result<(), Error> {
        while let Some(event) = receiver.recv().await {
            let data = self.serializer.serialize(&event)?;

            let message = Message::new(data);

            self.broker.connection().send(message).await?;

            let mut listeners = self.listeners.lock().await;

            for listener in listeners.iter_mut() {
                if event.name() == listener.event_name() {
                    let events = listener.on_event(event.clone()).await?;

                    for event in events {
                        let mut guard = self.queue.lock().await;

                        guard.send(event)
                            .await
                            .map_err(|e|
                                Error::Server(
                                    InternalServerError {
                                        context: Some(e.to_string().into())
                                    }
                                )
                            )?;
                    }
                }
            }
        }

        Ok(())
    }
}