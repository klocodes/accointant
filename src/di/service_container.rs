use std::sync::Arc;
use crate::config::manager::ConfigManager;
use crate::db::connection::manager::ConnectionManager;
use crate::db2::connection::manager::ConnectionManager as ConnectionManager2;
use crate::db::db_manager::DbManager;
use crate::errors::Error;
use crate::mq::manager::MqManager;
use crate::services::hasher::{BcryptHasher, Hasher};
use crate::services::http_client::{HttpClient, ReqwestClient};
use crate::services::jwt::{JsonwebtokenLibService, JwtService};
use crate::services::mailer::{LettreMailer, Mailer};
use crate::services::serializer::Serializer;
use crate::services::templater::{HandlebarsTemplater, Templater};
use crate::services::tokenizer::{SymbolsTokenizer, Tokenizer};

pub struct ServiceContainer {
    config: ConfigManager,
    db_manager: Arc<DbManager>,
    mq_manager: Arc<MqManager>,
    db_connection: Arc<Box< dyn crate::db2::connection::manager::ConnectionManager>>,
}

impl ServiceContainer {
    pub async fn new(
        config: ConfigManager,
        db_connection: Arc<Box<dyn ConnectionManager2>>,
    ) -> Result<Self, Error> {
        let db_manager = DbManager::new(config.db()).await?;
        let mq_manager = MqManager::new(config.mq()).await?;


        Ok(Self {
            config: config.clone(),
            db_manager: Arc::new(db_manager),
            mq_manager: Arc::new(mq_manager),
            db_connection: db_connection.clone(),
        })
    }

    pub fn config(&self) -> &ConfigManager {
        &self.config
    }

    pub fn db_manager(&self) -> Arc<DbManager> {
        self.db_manager.clone()
    }

    pub fn hasher(&self) -> impl Hasher {
        BcryptHasher::new()
    }

    pub async fn http_client(&self) -> impl HttpClient {
        ReqwestClient::new()
    }

    pub fn jwt_service(&self) -> impl JwtService {
        JsonwebtokenLibService::new(self.config.auth().clone())
    }

    pub fn mailer(&self) -> Result<impl Mailer, Error> {
        LettreMailer::new(self.config.mailer())
    }

    pub fn mq_manager(&self) -> Arc<MqManager> {
        self.mq_manager.clone()
    }

    pub fn serializer(&self) -> Serializer {
        Serializer::Cbor
    }

    pub fn templater(&self) -> Result<impl Templater, Error> {
        let templater = HandlebarsTemplater::new(self.config.templater().clone());

        Ok(templater)
    }

    pub fn tokenizer(&self) -> impl Tokenizer {
        SymbolsTokenizer::new()
    }
}