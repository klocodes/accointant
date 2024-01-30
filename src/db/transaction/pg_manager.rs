use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres, Transaction};
use crate::db::transaction::manager::TransactionManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Debug)]
pub struct PgTransactionManager<'a> {
    tx: Option<Transaction<'a, Postgres>>,
}

impl<'a> PgTransactionManager<'a> {
    pub fn new() -> Self {
        Self { tx: None }
    }
}

#[async_trait]
impl<'a> TransactionManager for PgTransactionManager<'a> {
    type Pool = Pool<Postgres>;

    type Transaction = Transaction<'a, Postgres>;

    async fn begin(&mut self, pool: Self::Pool) -> Result<(), Error> {
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

    async fn get(&mut self) -> Result<&mut Self::Transaction, Error> {
        let tx = self.tx.as_mut().ok_or(
            Error::Server(
                InternalServerError {
                    context: Some("Transaction has not started".into())
                }
            )
        )?;

        Ok(tx)
    }

    async fn commit(mut self) -> Result<(), Error> {
        let tx = self.tx.ok_or(
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

        self.tx = None;

        Ok(())
    }

    async fn rollback(mut self) -> Result<(), Error> {
        let tx = self.tx.ok_or(
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

        self.tx = None;

        Ok(())
    }
}