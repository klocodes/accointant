use crate::config::structs::mq::MqConfig;
use crate::errors::Error;
use crate::mq::connection::AmqpConnection;

pub struct MqManager {
    connection: AmqpConnection,
}

impl MqManager {
    pub async fn new(cfg: &MqConfig) -> Result<Self, Error> {
        let url = format!(
            "{}://{}:{}@{}:{}",
            cfg.driver(),
            cfg.username(),
            cfg.password(),
            cfg.host(),
            cfg.port(),
        );

        let connection = AmqpConnection::new(&url).await?;

        Ok(Self {
            connection,
        })
    }

    pub fn connection(&self) -> &AmqpConnection {
        &self.connection
    }
}