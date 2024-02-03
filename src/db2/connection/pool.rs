use std::sync::Arc;
use sqlx::{Pool as SqlxPool, Postgres};

pub trait ConnectionPool {}

impl ConnectionPool for SqlxPool<Postgres> {}

#[derive(Clone, Debug)]
pub enum Pool {
    Pg(Arc<SqlxPool<Postgres>>)
}

impl Pool {
    pub fn get(&self) -> Arc<impl ConnectionPool> {
        match self {
            Pool::Pg(pool) => pool.clone()

        }
    }
}