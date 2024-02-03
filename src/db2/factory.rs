use crate::config::structs::db::DbConfig;
use crate::db2::connection::manager::ConnectionManager;
use crate::db2::connection::pg_manager::PgConnectionManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

pub struct DbFactory;

impl DbFactory {
    pub async fn create_connection(cfg: &DbConfig) -> Result<impl ConnectionManager, Error>
    {
        let url = format!("postgres://{}:{}@{}:{}/{}",
                          cfg.user(),
                          cfg.password(),
                          cfg.host(),
                          cfg.port(),
                          cfg.name()
        );

        match cfg.driver() {
            "postgres" => {
                let mut connection_manager = PgConnectionManager::new();
                connection_manager.connect(&url, cfg.timeout(), cfg.max_connections()).await?;

                Ok(connection_manager)
            }
            _ => {
                Err(Error::Server(InternalServerError {
                    context: Some("Unsupported database driver".into())
                }))
            }
        }
    }
}
