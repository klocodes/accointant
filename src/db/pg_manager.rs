use std::time::Duration;
use async_trait::async_trait;
use sqlx::{ConnectOptions, PgPool, Postgres};
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPoolOptions};

use crate::db::db_manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Clone)]
pub struct PgManager {
    pool: Option<PgPool>,
}

impl PgManager {
    pub fn new() -> Self {
        Self {
            pool: None
        }
    }
}

#[async_trait]
impl DbManager for PgManager {
    type Pool = PgPool;
    type Connection = PoolConnection<Postgres>;

    async fn connect(&self, url: &str, timeout: Duration, max_connections: u32) -> Result<Self, Error> {
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

        Ok(Self {
            pool: Some(pool)
        })
    }

    async fn pool(&self) -> Result<Self::Pool, Error> {
        self.pool.clone().ok_or(Error::Server(InternalServerError {
            context: Some("Pool is not initialized".into()),
        }))
    }

    async fn connection(&self) -> Result<Self::Connection, Error> {
        let pool = self.pool().await?;

        let connection = pool.acquire().await.map_err(|e| {
            Error::Server(InternalServerError {
                context: Some(
                    format!("Failed to acquire connection from pool: {}", e.to_string()).into()
                )
            })
        })?;

        Ok(connection)
    }
}