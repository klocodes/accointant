use std::time::Duration;
use sqlx::{Pool, Postgres, Transaction};
use sqlx::postgres::PgPoolOptions;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Debug)]
pub struct PgManager {
    pool: Pool<Postgres>,
    tx: Option<Transaction<'static, Postgres>>,
}

impl PgManager {
    pub async fn connect(url: &str, timeout: Duration, max_connections: u32) -> Result<Self, Error> {
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
            pool,
            tx: None,
        })
    }

    pub fn pool(&self) -> Result<Pool<Postgres>, Error> {
        Ok(self.pool.clone())
    }

    pub(crate) async fn begin(&mut self, pool: Pool<Postgres>) -> Result<(), Error> {
        let tx = pool.begin().await.map_err(|e| {
            Error::Server(InternalServerError {
                context: Some(
                    format!("Failed to begin transaction: {}", e.to_string()).into()
                )
            })
        })?;

        self.tx = Some(tx);

        Ok(())
    }

    pub async fn transaction(&mut self) -> Result<&mut Transaction<'static, Postgres>, Error> {
        let tx = self.tx.as_mut().ok_or(
            Error::Server(
                InternalServerError {
                    context: Some("Transaction has not started".into())
                }
            )
        )?;

        Ok(tx)
    }

    pub async fn commit(&mut self) -> Result<(), Error> {
        let tx = self.tx.take().ok_or(
            Error::Server(
                InternalServerError {
                    context: Some("Transaction has not started".into())
                }
            )
        )?;

        tx.commit().await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to commit transaction: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }

    pub async fn rollback(&mut self) -> Result<(), Error> {
        let tx = self.tx.take().ok_or(
            Error::Server(
                InternalServerError {
                    context: Some("Transaction has not started".into())
                }
            )
        )?;

        tx.rollback().await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to rollback transaction: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }
}