use crate::config::structs::db::DbConfig;
use crate::db::manager::DbManager;
use crate::db::pg_manager::PgManager;
use crate::errors::Error;

pub struct DbFactory;

impl DbFactory {
    pub async fn create(cfg: &DbConfig) -> Result<DbManager, Error> {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                          cfg.user(),
                          cfg.password(),
                          cfg.host(),
                          cfg.port(),
                          cfg.name()
        );

        let pg_manager = PgManager::connect(&url, cfg.timeout(), cfg.max_connections()).await?;

        Ok(DbManager::Pg(pg_manager))
    }
}