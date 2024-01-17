use crate::config::db::DbConfig;
use crate::db::manager::db_manager;
use crate::db::manager::db_manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors;

pub struct DbFactory;

impl DbFactory {
    pub async fn create(cfg: &DbConfig) -> Result<DbManager, Error> {
        let db_url = format!("{}://{}:{}@{}:{}/{}", cfg.driver(), cfg.user(), cfg.password(), cfg.host(), cfg.port(), cfg.name());

        let db_manager = db_manager::DbManager::new();

        Ok(db_manager.connect(db_url.as_str(), cfg.max_connections(), cfg.timeout()).await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!(
                    "Failed to connect to database: {}", e.to_string()
                ).into())
            }))?)
    }
}