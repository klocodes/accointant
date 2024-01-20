use crate::config::db::DbConfig;
use crate::db::manager::db_manager::DbManager;
use crate::db::manager::pg_manager::PgManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors;

pub struct DbManagerFactory;

impl DbManagerFactory {
    pub fn create(cfg: &DbConfig) -> Result<PgManager, Error>
    {
        let conn_str = format!("host={} port={} user={} password={} dbname={}",
                               cfg.host(),
                               cfg.port(),
                               cfg.user(),
                               cfg.password(),
                               cfg.name()
        );

        PgManager::new().connect(&conn_str)
    }
}