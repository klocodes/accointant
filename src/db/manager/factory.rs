use crate::config::db::DbConfig;
use crate::db::manager::db_manager::DbManager;
use crate::db::manager::pg_manager;
use crate::db::manager::pg_manager::PgManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors;

pub struct DbManagerFactory;

impl DbManagerFactory {
    pub async fn create(cfg: &DbConfig) -> Result<PgManager, Error>
    {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                               cfg.host(),
                               cfg.port(),
                               cfg.user(),
                               cfg.password(),
                               cfg.name()
        );

        let pg_manager = PgManager::new().connect(&url, cfg.timeout(), cfg.max_connections()).await?;

        Ok(pg_manager)
    }
}