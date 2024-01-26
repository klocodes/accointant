use crate::config::structs::db::DbConfig;
use crate::db::connection::manager::ConnectionManager;
use crate::db::connection::pg_manager::PgConnectionManager;
use crate::errors::Error;

pub struct ConnectionManagerFactory;

impl ConnectionManagerFactory {
    pub async fn create(cfg: &DbConfig) -> Result<PgConnectionManager, Error>
    {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                          cfg.user(),
                          cfg.password(),
                          cfg.host(),
                          cfg.port(),
                          cfg.name()
        );

        let pg_manager = PgConnectionManager::new().connect(&url, cfg.timeout(), cfg.max_connections()).await?;

        Ok(pg_manager)
    }
}