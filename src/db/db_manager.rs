use std::sync::Arc;
use crate::config::structs::db::DbConfig;
use crate::db::connection::manager::ConnectionManager;
use crate::db::connection::pg_manager::PgConnectionManager;
use crate::db::transaction::container::TransactionContainer;
use crate::errors::Error;

#[derive(Clone)]
pub struct DbManager {
    conn: Arc<PgConnectionManager>,
}

impl DbManager {
    pub async fn new(cfg: &DbConfig) -> Result<Self, Error>
    {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                          cfg.user(),
                          cfg.password(),
                          cfg.host(),
                          cfg.port(),
                          cfg.name()
        );

        let connection_manager = PgConnectionManager::new().connect(&url, cfg.timeout(), cfg.max_connections()).await?;

        Ok(Self {
            conn: Arc::new(connection_manager)
        })
    }

    pub async fn connection_manager(&self) -> Result<Arc<PgConnectionManager>, Error> {
        Ok(self.conn.clone())
    }

    pub fn transaction_container(&self) -> Result<TransactionContainer, Error> {
        Ok(TransactionContainer::new())
    }
}
