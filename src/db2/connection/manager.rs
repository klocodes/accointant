use std::time::Duration;
use async_trait::async_trait;
use crate::db2::connection::pool::Pool;
use crate::errors::Error;

#[async_trait]
pub trait ConnectionManager: Send + Sync {
    async fn connect(&mut self, url: &str, timeout: Duration, max_connections: u32) -> Result<(), Error>
        where Self: Sized;

    async fn pool(&self) -> Result<Pool, Error>;
}
