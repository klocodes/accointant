use sqlx::{Pool, Postgres, Transaction};
use crate::db::error::DbError;
use crate::db::pg_manager::PgManager;

#[derive(Debug)]
pub enum DbManager {
    Pg(PgManager),
    Mock(MockManager),
}

impl DbManager {
    pub fn pool(&self) -> Result<Pool<Postgres>, DbError> {
        match self {
            Self::Pg(manager) => manager.pool(),
            Self::Mock(_) => Err(
                DbError::MockPool
            ),
        }
    }

    pub async fn begin(&mut self) -> Result<(), DbError> {
        match self {
            Self::Pg(manager) => manager.begin(manager.pool()?).await,
            Self::Mock(_) => Ok(()),
        }
    }

    pub async fn transaction(&mut self) -> Result<&mut Transaction<'static, Postgres>, DbError> {
        match self {
            Self::Pg(manager) => manager.transaction().await,
            Self::Mock(_) => Err(
               DbError::Transaction("Transaction has not started".to_string())
            ),
        }
    }

    pub async fn commit(&mut self) -> Result<(), DbError> {
        match self {
            Self::Pg(manager) => manager.commit().await,
            Self::Mock(_) => Ok(()),
        }
    }

    pub async fn rollback(&mut self) -> Result<(), DbError> {
        match self {
            Self::Pg(manager) => manager.rollback().await,
            Self::Mock(_) => Ok(()),
        }
    }
}


#[derive(Debug)]
pub struct MockManager {
    has_error: bool,
}

impl MockManager {
    pub fn new(has_error: bool) -> Self {
        Self { has_error }
    }

    pub async fn commit(&mut self) -> Result<(), DbError> {
        if self.has_error {
            Err(
                DbError::Mock
            )
        } else {
            Ok(())
        }
    }

    pub async fn rollback(&mut self) -> Result<(), DbError> {
        if self.has_error {
            Err(
               DbError::Mock
            )
        } else {
            Ok(())
        }
    }
}




