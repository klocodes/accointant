use std::time::Duration;
use sqlx::{Pool, Postgres, Transaction};
use sqlx::postgres::PgPoolOptions;
use crate::db::error::DbError;

#[derive(Debug)]
pub struct PgManager {
    pool: Pool<Postgres>,
    tx: Option<Transaction<'static, Postgres>>,
}

impl PgManager {
    pub async fn connect(url: &str, timeout: Duration, max_connections: u32) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new()
            .acquire_timeout(timeout)
            .max_connections(max_connections)
            .connect(url)
            .await
            .map_err(|e| {
                DbError::Connection(e.to_string())
            })?;

        Ok(Self {
            pool,
            tx: None,
        })
    }

    pub fn pool(&self) -> Result<Pool<Postgres>, DbError> {
        Ok(self.pool.clone())
    }

    pub(crate) async fn begin(&mut self, pool: Pool<Postgres>) -> Result<(), DbError> {
        let tx = pool.begin().await.map_err(|e| {
            DbError::Transaction(
                format!("Failed to begin transaction. {}", e.to_string())
            )
        })?;

        self.tx = Some(tx);

        Ok(())
    }

    pub async fn transaction(&mut self) -> Result<&mut Transaction<'static, Postgres>, DbError> {
        let tx = self.tx.as_mut().ok_or(
            DbError::Transaction("Transaction has not started".to_string())
        )?;

        Ok(tx)
    }

    pub async fn commit(&mut self) -> Result<(), DbError> {
        let tx = self.tx.take().ok_or(
            DbError::Transaction("Transaction has not started".to_string())
        )?;

        tx.commit().await.map_err(|e| {
            DbError::Transaction(
                format!("Failed to commit transaction. {}", e.to_string())
            )
        })?;

        Ok(())
    }

    pub async fn rollback(&mut self) -> Result<(), DbError> {
        let tx = self.tx.take().ok_or(
            DbError::Transaction("Transaction has not started".to_string())
        )?;

        tx.rollback().await.map_err(|e| {
            DbError::Transaction(
                format!("Failed to rollback transaction. {}", e.to_string())
            )
        })?;

        Ok(())
    }
}