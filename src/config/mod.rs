use serde::Deserialize;
use config::{Config as ConfigLoader, Environment, File, FileFormat};
use dotenv::dotenv;

pub mod actor;
pub mod actor_error;
pub mod db;
pub mod general;
pub mod server;
pub mod log;

const GENERAL_CONFIG_PATH: &str = "config/general.toml";
const DB_CONFIG_PATH: &str = "config/db.toml";
const SERVER_CONFIG_PATH: &str = "config/server.toml";
const LOG_CONFIG_PATH: &str = "config/log.toml";

#[derive(Deserialize, Clone)]
pub struct Config {
    general: general::GeneralConfig,
    db: db::DbConfig,
    server: server::ServerConfig,
    log: log::LogConfig,
}

impl Config {
    pub fn new() -> Self {
        let project_root = std::env::var("PROJECT_ROOT")
            .expect("The PROJECT_ROOT environment variable is not set");

        dotenv().ok();

        let general_config_path = format!("{}/{}", project_root, GENERAL_CONFIG_PATH);
        let db_config_path = format!("{}/{}", project_root, DB_CONFIG_PATH);
        let server_config_path = format!("{}/{}", project_root, SERVER_CONFIG_PATH);
        let log_config_path = format!("{}/{}", project_root, LOG_CONFIG_PATH);


        let builder = ConfigLoader::builder()
            .add_source(File::new(&general_config_path, FileFormat::Toml))
            .add_source(File::new(&db_config_path, FileFormat::Toml))
            .add_source(File::new(&server_config_path, FileFormat::Toml))
            .add_source(File::new(&log_config_path, FileFormat::Toml))
            .add_source(Environment::default()
                            .try_parsing(true)
                            .separator("_"),
            );

        builder.build()
            .and_then(ConfigLoader::try_deserialize)
            .map_err(|e| format!("Failed to load and deserialize configuration: {}", e))
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn general(&self) -> &general::GeneralConfig {
        &self.general
    }

    pub fn db(&self) -> &db::DbConfig {
        &self.db
    }

    pub fn server(&self) -> &server::ServerConfig {
        &self.server
    }

    pub fn log(&self) -> &log::LogConfig {
        &self.log
    }
}