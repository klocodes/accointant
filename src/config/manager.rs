use config::{Config as ConfigLoader, Environment, File, FileFormat};
use dotenv::dotenv;
use serde::Deserialize;
use crate::config::structs::auth::AuthConfig;

use crate::config::structs::db::DbConfig;
use crate::config::structs::general::GeneralConfig;
use crate::config::structs::log::LogConfig;
use crate::config::structs::mailer::MailerConfig;
use crate::config::structs::mq::MqConfig;
use crate::config::structs::server::ServerConfig;
use crate::config::structs::templater::TemplaterConfig;

const CONFIG_DIR: &str = "config";
const CONFIG_FORMAT: FileFormat = FileFormat::Toml;

#[derive(Deserialize, Clone, Debug)]
pub struct ConfigManager {
    auth: AuthConfig,
    db: DbConfig,
    general: GeneralConfig,
    log: LogConfig,
    mailer: MailerConfig,
    mq: MqConfig,
    server: ServerConfig,
    templater: TemplaterConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        //Get project root path from PROJECT_ROOT environment variable
        let project_root = std::env::var("PROJECT_ROOT")
            .expect("The PROJECT_ROOT environment variable is not set");

        dotenv().ok();

        //Define configuration file names
        //TODO: Add configuration file names to here
        let files = vec![
            "auth.toml",
            "db.toml",
            "general.toml",
            "log.toml",
            "mailer.toml",
            "server.toml",
            "templater.toml",
        ];

        //Init configuration builder from Config library
        let mut builder = ConfigLoader::builder();

        //Add configuration files to builder
        for file in files {
            let path = format!("{}/{}/{}", project_root, CONFIG_DIR, file);
            if !std::path::Path::new(&path).exists() {
                panic!("Configuration file not found: {}", path);
            }

            builder = builder.add_source(File::new(&path, CONFIG_FORMAT))
        }

        //Add environment variables to builder
        builder = builder.add_source(Environment::default()
                                         .try_parsing(true)
                                         .separator("_"),
        );

        //Build configuration
        builder.build()
            .and_then(ConfigLoader::try_deserialize)
            .map_err(|e| format!("Failed to load and deserialize configuration: {}", e))
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }

    pub fn db(&self) -> &DbConfig {
        &self.db
    }

    pub fn general(&self) -> &GeneralConfig {
        &self.general
    }

    pub fn log(&self) -> &LogConfig {
        &self.log
    }

    pub fn mailer(&self) -> &MailerConfig {
        &self.mailer
    }
    pub fn mq(&self) -> &MqConfig {
        &self.mq
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn templater(&self) -> &TemplaterConfig {
        &self.templater
    }
}
