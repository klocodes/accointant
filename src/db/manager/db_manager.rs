use std::format;
use std::time::Duration;
use async_trait::async_trait;

use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[async_trait]
pub trait DbManager: Clone {
    type Pool;
    type Connection;

    async fn connect(&self, url: &str, timeout: Duration, max_connections: u32) -> Result<Self, Error>;

    async fn get_pool(&self) -> Result<Self::Pool, Error>;
    async fn get_connection(&self) -> Result<Self::Connection, Error>;
}