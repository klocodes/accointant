use sqlx::Postgres;
use tracing_appender::non_blocking::WorkerGuard;

use crate::config::manager::ConfigManager as Config;
use crate::db::connection::factory::ConnectionManagerFactory;
use crate::db::connection::pg_manager::PgConnectionManager;
use crate::db::db_transaction::PgTransaction;
use crate::errors::Error;
use crate::services::mailer::{LettreMailer, Mailer};

#[derive(Clone)]
pub struct AppContext {
    config: Config,
    dm_manager: PgConnectionManager,
    mailer: LettreMailer,
}

impl AppContext {
    pub async fn new() -> Result<(Self, WorkerGuard), Error> {
        let config = Config::new();

        let log_config = config.log();
        let db_config = config.db();

        let _guard = crate::log::logger::init(log_config).await.unwrap();

        let dm_manager = ConnectionManagerFactory::create(db_config).await?;

        let mailer = LettreMailer::new(config.mailer())?;

        Ok((Self {
            config,
            dm_manager,
            mailer,
        }, _guard))
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_db_manager(&self) -> &PgConnectionManager {
        &self.dm_manager
    }

    pub fn get_mailer(&self) -> &LettreMailer {
        &self.mailer
    }
}

pub type TransactionManager = PgTransaction<'static>;