use tracing_appender::non_blocking::WorkerGuard;

use crate::config::Config;
use crate::db::db_transaction::PgTransaction;
use crate::db::factory::DbManagerFactory;
use crate::db::pg_manager::PgManager;
use crate::errors::Error;

#[derive(Clone)]
pub struct AppContext {
    config: Config,
    dm_manager: PgManager,
}

impl AppContext {
    pub async fn new() -> Result<(Self, WorkerGuard), Error> {
        let config = Config::new();

        let log_config = config.log();
        let db_config = config.db();

        let _guard = crate::log::logger::init(log_config).await.unwrap();

        let dm_manager = DbManagerFactory::create(db_config).await?;

        Ok((Self {
            config,
            dm_manager,
        }, _guard))
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_db_manager(&self) -> &PgManager {
        &self.dm_manager
    }
}

pub type TransactionManager = PgTransaction<'static>;