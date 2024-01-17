use std::sync::Arc;
use std::time::Duration;
use futures_util::lock::Mutex;
use sqlx::{Database, Encode, Pool, Postgres, Row, Transaction, Type};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::errors::Error;
use crate::errors::server::ServerErrors;

pub(crate) type DbType = Postgres;
type DbPoolOptions = PgPoolOptions;
pub type DbPool = Pool<DbType>;
pub(crate) type DbTransaction<'a> = Transaction<'a, Postgres>;
pub(crate) type DbRow = PgRow;

#[derive(Clone)]
pub struct DbManager {
    pool: Arc<Mutex<Option<DbPool>>>,
}

impl DbManager {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&self, db_url: &str, max_connections: u32, timeout: Duration) -> Result<Self, Error> {
        let pool = DbPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(timeout)
            .connect(db_url)
            .await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!(
                    "Failed to get pool: {}", e.to_string()
                ).into())
            }))?;

        Ok(Self {
            pool: Arc::new(Mutex::new(Some(pool))),
        })
    }

    pub async fn begin_transaction(mut self, pool: &DbPool) -> Result<Transaction<'_, DbType>, Error> {
        pool.begin().await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!(
                    "Failed to get transaction from pool: {}", e.to_string()
                ).into())
            }))
    }

    pub async fn commit(transaction: Transaction<'_, DbType>) -> Result<(), Error> {
        transaction.commit().await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!(
                    "Failed to commit transaction: {}", e.to_string()
                ).into())
            }))?;

        Ok(())
    }

    pub async fn rollback(transaction: Transaction<'_, DbType>) -> Result<(), Error> {
        transaction.rollback().await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!(
                    "Failed to rollback transaction: {}", e.to_string()
                ).into())
            }))?;

        Ok(())
    }

    pub async fn execute_query<'q, A, R>(&self, query: &'q str, args: &'q Vec<A>) -> Result<Vec<R>, Error>
        where
            A: 'q + Send + Encode<'q, DbType> + Type<DbType> + Sync,
            R: for<'r> sqlx::FromRow<'r, DbRow> + Unpin + Send,
    {
        let mut sqlx_query = sqlx::query_as::<_, R>(query);
        for arg in args {
            sqlx_query = sqlx_query.bind(arg);
        }

        let pool = self.pool.lock().await; // Блокировка мьютекса
        let pool = pool.as_ref().ok_or(Error::Server(ServerErrors::InternalServerError {
            context: Some("Database pool is not available".into())
        }))?;

        let rows = sqlx_query
            .fetch_all(&*pool)
            .await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(
                    format!("Failed to execute query: {}", e.to_string()).into()
                )
            }))?;

        Ok(rows)
    }

    pub async fn get_pool(&self) -> Result<DbPool, Error> {
        let pool = self.pool.lock().await; // Блокировка мьютекса
        let pool = pool.as_ref().ok_or(Error::Server(ServerErrors::InternalServerError {
            context: Some("Database pool is not available".into())
        }))?;

        Ok(pool.clone())
    }
}