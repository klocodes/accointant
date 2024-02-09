use sqlx::{Pool, Postgres, Transaction};
use crate::db::pg_manager::PgManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Debug)]
pub enum DbManager {
    Pg(PgManager),
    Mock(MockManager),
}

impl DbManager {
    pub fn pool(&self) -> Result<Pool<Postgres>, Error> {
        match self {
            Self::Pg(manager) => manager.pool(),
            Self::Mock(_) => Err(
                Error::Server(
                    InternalServerError {
                        context: Some("Mock database manager does not have a pool".into())
                    }
                )
            ),
        }
    }

    pub async fn begin(&mut self) -> Result<(), Error> {
        match self {
            Self::Pg(manager) => manager.begin(manager.pool()?).await,
            Self::Mock(_) => Ok(()),
        }
    }

    pub async fn transaction(&mut self) -> Result<&mut Transaction<'static, Postgres>, Error> {
        match self {
            Self::Pg(manager) => manager.transaction().await,
            Self::Mock(_) => Err(
                Error::Server(
                    InternalServerError {
                        context: Some("Transaction has not started".into())
                    }
                )
            ),
        }
    }

    pub async fn commit(&mut self) -> Result<(), Error> {
        match self {
            Self::Pg(manager) => manager.commit().await,
            Self::Mock(_) => Ok(()),
        }
    }

    pub async fn rollback(&mut self) -> Result<(), Error> {
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

    pub async fn commit(&mut self) -> Result<(), Error> {
        if self.has_error {
            Err(
                Error::Server(
                    InternalServerError {
                        context: Some("Mock database manager has an error".into())
                    }
                )
            )
        } else {
            Ok(())
        }
    }

    pub async fn rollback(&mut self) -> Result<(), Error> {
        if self.has_error {
            Err(
                Error::Server(
                    InternalServerError {
                        context: Some("Mock database manager has an error".into())
                    }
                )
            )
        } else {
            Ok(())
        }
    }
}




