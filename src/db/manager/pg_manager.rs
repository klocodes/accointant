use postgres::{Config, NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use crate::db::manager::db_manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Clone)]
pub struct PgManager {
    pool: Option<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PgManager {
    pub fn new() -> Self {
        Self {
            pool: None
        }
    }
}

impl DbManager for PgManager {
    type Pool = Pool<PostgresConnectionManager<NoTls>>;

    fn connect(&self, conn_str: &str) -> Result<Self, Error> {
        let config = conn_str.parse::<Config>()
            .map_err(|e| Error::Server(InternalServerError {
                context: Some(e.to_string().into())
            }))?;

        let pool = Pool::new(PostgresConnectionManager::new(config, NoTls)).map_err(|e| {
            Error::Server(InternalServerError {
                context: Some(e.to_string().into())
            })
        })?;

        Ok(Self {
            pool: Some(pool)
        })
    }

    fn pool(&self) -> Result<Self::Pool, Error> {
        self.pool.clone().ok_or(Error::Server(InternalServerError {
            context: Some("Pool is not initialized".into())
        }))
    }
}