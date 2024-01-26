use crate::config::structs::db::DbConfig;
use crate::db::db_manager::DbManager;
use crate::db::pg_manager::PgManager;
use crate::errors::Error;

pub struct DbManagerFactory;

impl DbManagerFactory {
    pub async fn create(cfg: &DbConfig) -> Result<PgManager, Error>
    {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                          cfg.user(),
                          cfg.password(),
                          cfg.host(),
                          cfg.port(),
                          cfg.name()
        );

        let pg_manager = PgManager::new().connect(&url, cfg.timeout(), cfg.max_connections()).await?;

        Ok(pg_manager)
    }
}