use std::sync::Arc;
use crate::config::manager::ConfigManager;
use crate::db::connection::manager::ConnectionManager;
use crate::db::db_manager::DbManager;
use crate::db::transaction::pg_manager::PgTransactionManager;
use crate::errors::Error;
use crate::services::hasher::{BcryptHasher, Hasher};
use crate::services::jwt::{JsonwebtokenLibService, JwtService};
use crate::services::mailer::{LettreMailer, Mailer};
use crate::services::serializer::{CborSerializer, Serializer};
use crate::services::templater::{HandlebarsTemplater, Templater};
use crate::services::tokenizer::{SymbolsTokenizer, Tokenizer};

#[derive(Clone, Debug)]
pub struct ServiceContainer {
    config: ConfigManager,
    db_manager: DbManager,
}

impl ServiceContainer {
    pub async fn new(config: ConfigManager) -> Result<Self, Error> {
        let db_manager = DbManager::new(config.db()).await?;


        Ok(Self {
            config: config.clone(),
            db_manager
        })
    }

    pub fn config(&self) -> &ConfigManager {
        &self.config
    }

    pub fn db_manager(&self) -> DbManager {
        self.db_manager.clone()
    }

    pub fn hasher(&self) -> impl Hasher {
        BcryptHasher::new()
    }

    pub fn jwt_service(&self) -> impl JwtService {
        JsonwebtokenLibService::new(self.config.auth().clone())
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