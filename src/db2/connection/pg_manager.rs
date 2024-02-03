use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use crate::db2::connection::manager::ConnectionManager;
use crate::db2::connection::pool::Pool;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Clone, Debug)]
pub struct PgConnectionManager {
    pool: Option<Pool>,
}

impl PgConnectionManager {
    pub fn new() -> Self {
        Self {
            pool: None
        }
    }
}

#[async_trait]
impl ConnectionManager for PgConnectionManager {
    async fn connect(&mut self, url: &str, timeout: Duration, max_connections: u32) -> Result<(), Error> {
        let pool = PgPoolOptions::new()
            .acquire_timeout(timeout)
            .max_connections(max_connections)
            .connect(url)
            .await
            .map_err(|e| {
                Error::Server(InternalServerError {
                    context: Some(
                        format!("Failed to connect to database: {}", e.to_string()).into()
                    )
                })
            })?;

        let pool = Arc::new(pool);

        self.pool = Some(
            Pool::Pg(pool.clone())
        );

        Ok(())
    }

    async fn pool(&self) -> Result<Pool, Error> {
        let pool = self.pool.clone().ok_or(Error::Server(InternalServerError {
            context: Some("Pool is not initialized".into()),
        }))?;

        Ok(pool)
    }
}