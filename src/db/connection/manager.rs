use std::time::Duration;
use async_trait::async_trait;

use crate::errors::Error;

#[async_trait]
pub trait ConnectionManager: Clone {
    type Pool;
    type Connection;

    async fn connect(&self, url: &str, timeout: Duration, max_connections: u32) -> Result<Self, Error>;
    async fn pool(&self) -> Result<Self::Pool, Error>;
    async fn connection(&self) -> Result<Self::Connection, Error>;
}