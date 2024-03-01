use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::manager::ConfigManager;
use crate::db::factory::DbFactory;
use crate::db::manager::DbManager;
use crate::di::error::ServiceContainerError;
use crate::mq::manager::MqManager;
use crate::services::hasher::{BcryptHasher, Hasher};
use crate::services::http_client::{HttpClient, ReqwestClient};
use crate::services::jwt::{JsonwebtokenLibService, JwtService};
use crate::services::mailer::{LettreMailer, Mailer};
use crate::services::serializer::Serializer;
use crate::services::templater::{HandlebarsTemplater, Templater};
use crate::services::tokenizer::{SymbolsTokenizer, Tokenizer};
use crate::support::command_bus::{Command, CommandBus, CommandHandler};

pub struct ServiceContainer {
    config: ConfigManager,
    db_manager: Arc<Mutex<DbManager>>,
    mq_manager: Arc<MqManager>,
}

impl ServiceContainer {
    pub async fn new(
        config: ConfigManager,
    ) -> Result<Self, ServiceContainerError> {
        let db_manager = DbFactory::create(config.db()).await
            .map_err(|e| ServiceContainerError::DbConnection(e.to_string()))?;
        let mq_manager = MqManager::new(config.mq()).await
            .map_err(|e| ServiceContainerError::MqConnection(e.to_string()))?;


        Ok(Self {
            config: config.clone(),
            db_manager: Arc::new(Mutex::new(db_manager)),
            mq_manager: Arc::new(mq_manager),
        })
    }

    pub fn config(&self) -> &ConfigManager {
        &self.config
    }

    pub fn command_bus<C, H>(&self) -> CommandBus<C, H>
        where
            C: 'static + Send + Sync + Command,
            H: 'static + Send + Sync + CommandHandler<C>,
    {
        CommandBus::new()
    }

    pub fn db_manager(&self) -> Arc<Mutex<DbManager>> {
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

    pub fn mailer(&self) -> impl Mailer {
        LettreMailer::new(self.config.mailer())
    }

    pub fn mq_manager(&self) -> Arc<MqManager> {
        self.mq_manager.clone()
    }

    pub fn serializer(&self) -> Serializer {
        Serializer::Cbor
    }

    pub fn templater(&self) -> impl Templater {
        HandlebarsTemplater::new(self.config.templater().clone())
    }

    pub fn tokenizer(&self) -> impl Tokenizer {
        SymbolsTokenizer::new()
    }
}