use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Queue};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use tokio_executor_trait::Tokio;
use tokio_reactor_trait::Tokio as TokioReactor;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::mq::message::Message;

const QUEUE_NAME: &str = "default";
const CONSUMER_TAG: &str = "default";

#[async_trait]
pub trait MqConnection {
    async fn send(&self, message: Message) -> Result<(), Error>;

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), Error>) -> Result<(), Error>;
}

pub struct AmqpConnection {
    connection: Connection,
    channel: Channel,
    queue: Queue,
    consumer: Consumer,
}

impl AmqpConnection {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let connection = Connection::connect(
            url,
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

        Ok(Self { connection, channel, queue, consumer })
    }
}

#[async_trait]
impl MqConnection for AmqpConnection {
    async fn send(&self, message: Message) -> Result<(), Error> {

        let _ = self.channel.basic_publish(
            "",
            QUEUE_NAME,
            BasicPublishOptions::default(),
            message.data().as_slice(),
            BasicProperties::default(),
        ).await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(format!("Failed to publish message: {}", e.to_string()).into())
                }
            )
        });

        Ok(())
    }

    async fn consume(&mut self, callback: fn(message: Message) -> Result<(), Error>) -> Result<(), Error> {
        while let Some(delivery) = self.consumer.next().await {
            let delivery = delivery.map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to get next delivery: {}", e.to_string()).into())
                    }
                )
            })?;

            let message = Message::new(delivery.data.clone());

            let _result = callback(message)?;

            let _ = delivery.ack(BasicAckOptions::default()).await.map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to ack delivery: {}", e.to_string()).into())
                    }
                )
            })?;
        }

        Ok(())
    }
}