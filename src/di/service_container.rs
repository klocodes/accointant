use crate::config::manager::ConfigManager;
use crate::errors::Error;
use crate::services::hasher::{BcryptHasher, Hasher};
use crate::services::mailer::{LettreMailer, Mailer};
use crate::services::templater::{HandlebarsTemplater, Templater};
use crate::services::tokenizer::{SymbolsTokenizer, Tokenizer};

#[derive(Clone)]
pub struct ServiceContainer {
    config: ConfigManager,
}

impl ServiceContainer {
    pub fn new(config: ConfigManager) -> Self {
        Self { config }
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

    pub fn templater(&self) -> Result<impl Templater, Error> {
        let templater = HandlebarsTemplater::new(self.config.templater().clone());

        Ok(templater)
    }

    pub fn tokenizer(&self) -> impl Tokenizer {
        SymbolsTokenizer::new()
    }
}