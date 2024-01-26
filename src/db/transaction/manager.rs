use async_trait::async_trait;
use crate::errors::Error;

#[async_trait]
pub trait TransactionManager {
    type Pool;
    type Transaction;

    async fn begin(&mut self, pool: Self::Pool) -> Result<(), Error>;
    async fn get(&mut self) -> Result<&mut Self::Transaction, Error>;

    async fn commit(mut self) -> Result<(), Error>;

    async fn rollback(mut self) -> Result<(), Error>;
}