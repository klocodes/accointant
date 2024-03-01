use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Queue};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use tokio_executor_trait::Tokio;
use tokio_reactor_trait::Tokio as TokioReactor;
use crate::mq::error::MqError;
use crate::mq::message::Message;

const QUEUE_NAME: &str = "default";
const CONSUMER_TAG: &str = "default";

#[async_trait]
pub trait MqConnection {
    async fn send(&self, message: Message) -> Result<(), MqError>;

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), MqError>) -> Result<(), MqError>;
}

pub struct AmqpConnection {
    connection: Connection,
    channel: Channel,
    queue: Queue,
    consumer: Consumer,
}

impl AmqpConnection {
    pub async fn new(url: &str) -> Result<Self, MqError> {
        let connection = Connection::connect(
            url,
            ConnectionProperties::default()
                .with_executor(Tokio::current())
                .with_reactor(TokioReactor),
        ).await
            .map_err(|e| {
                MqError::Connection(e.to_string())
            })?;

        let channel = connection.create_channel().await.map_err(|e| {
            MqError::Channel(e.to_string())
        })?;

        let queue = channel.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| {
                MqError::Queue(e.to_string())
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

        Ok(Self { connection, channel, queue, consumer })
    }
}

#[async_trait]
impl MqConnection for AmqpConnection {
    async fn send(&self, message: Message) -> Result<(), MqError> {

        let _ = self.channel.basic_publish(
            "",
            QUEUE_NAME,
            BasicPublishOptions::default(),
            message.data().as_slice(),
            BasicProperties::default(),
        ).await.map_err(|e| {
            MqError::Sending(e.to_string())
        });

        Ok(())
    }

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), MqError>) -> Result<(), MqError> {
        while let Some(delivery) = self.consumer.next().await {
            let delivery = delivery.map_err(|e| {
                MqError::Consuming(e.to_string()
                )
            })?;

            let message = Message::new(delivery.data.clone());

            let _result = callback(message)?;

            let _ = delivery.ack(BasicAckOptions::default()).await.map_err(|e| {
                MqError::Consuming(e.to_string())
            })?;
        }

        Ok(())
    }
}