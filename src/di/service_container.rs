use std::sync::Arc;
use crate::config::manager::ConfigManager;
use crate::db::connection::factory::ConnectionManagerFactory;
use crate::db::connection::manager::ConnectionManager;
use crate::db::connection::pg_manager::PgConnectionManager;
use crate::errors::Error;
use crate::services::hasher::{BcryptHasher, Hasher};
use crate::services::mailer::{LettreMailer, Mailer};
use crate::services::serializer::{CborSerializer, Serializer};
use crate::services::templater::{HandlebarsTemplater, Templater};
use crate::services::tokenizer::{SymbolsTokenizer, Tokenizer};

#[derive(Clone)]
pub struct ServiceContainer {
    config: ConfigManager,
    connection_manager: Arc<PgConnectionManager>,
}

impl ServiceContainer {
    pub async fn new(config: ConfigManager) -> Result<Self, Error> {
        let connection_manager = ConnectionManagerFactory::create(config.db()).await?;

        let container = Self {
            config: config.clone(),
            connection_manager: Arc::new(connection_manager)
        };

        Ok(container)
    }

    pub fn config(&self) -> &ConfigManager {
        &self.config
    }

    pub fn hasher(&self) -> impl Hasher {
        BcryptHasher::new()
    }

    pub fn mailer(&self) -> Result<impl Mailer, Error> {
        LettreMailer::new(self.config.mailer())
    }

    pub fn serializer(&self) -> impl Serializer {
        CborSerializer::new()
    }

    pub fn templater(&self) -> Result<impl Templater, Error> {
        let templater = HandlebarsTemplater::new(self.config.templater().clone());

        Ok(templater)
    }

    pub fn tokenizer(&self) -> impl Tokenizer {
        SymbolsTokenizer::new()
    }
}