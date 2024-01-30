use std::sync::Arc;
use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{Channel, Connection, ConnectionProperties, Consumer, Queue};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use tokio::sync::{Mutex};
use tokio_executor_trait::Tokio;
use tokio_reactor_trait::Tokio as TokioReactor;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::{log_error, log_trace};
use crate::mq::message::Message;

const QUEUE_NAME: &str = "default";
const CONSUMER_TAG: &str = "default";

#[async_trait]
pub trait MqManager {
    fn send(&self, message: Message) -> Result<(), Error>;

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), Error>) -> Result<(), Error>;
}

pub struct AmqpManager {
    connection: Connection,
    channel: Arc<Mutex<Channel>>,
    queue: Queue,
    consumer: Consumer,
}

impl AmqpManager {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let connection = Connection::connect(
            uri,
            ConnectionProperties::default()
                .with_executor(Tokio::current())
                .with_reactor(TokioReactor),
        ).await
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to connect to MQ: {}", e.to_string()).into())
                    }
                )
            })?;

        let channel = connection.create_channel().await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(format!("Failed to create MQ channel: {}", e.to_string()).into())
                }
            )
        })?;

        let queue = channel.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to declare MQ queue: {}", e.to_string()).into())
                    }
                )
            })?;

        let consumer = channel
            .basic_consume(
                QUEUE_NAME,
                CONSUMER_TAG,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        let channel = Arc::new(Mutex::new(channel));

        Ok(Self { connection, channel, queue, consumer })
    }
}

#[async_trait]
impl MqManager for AmqpManager {
    fn send(&self, message: Message) -> Result<(), Error> {
        Ok(())
    }

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), Error>) -> Result<(), Error> {
        while let Some(delivery) = self.consumer.next().await {
            let delivery = delivery.expect("error caught in consumer");
            let message = Message::new(delivery.data.clone()); // Создание вашего сообщения

            if let Err(e) = callback(message) {
                log_trace!("Error processing message: {:?}", e);
                log_error!("Error processing message: {:?}", e);
            }

            let _ = delivery.ack(BasicAckOptions::default()).await.map_err(|e| {
                log_error!("Error acknowledging message: {:?}", e);
                log_trace!("Error acknowledging message: {:?}", e);
            });
        }

        Ok(())
    }
}