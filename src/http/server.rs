use std::sync::Arc;
use actix::Addr;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use tokio::sync::Mutex;

use crate::config::actor::ConfigActor;
use crate::config::server::ServerConfig;
use crate::db::manager::db_manager::DbManager;
use crate::http::handlers::auth;
use crate::http::handlers::errors::not_found;

pub async fn run(server_config: &ServerConfig, config_actor: Addr<ConfigActor>, db_manager: DbManager) -> std::io::Result<()> {
    HttpServer::new(move || {
        let db_manager = db_manager.clone();

        App::new()
            //.wrap(ErrorHandling)
            .app_data(Data::new(config_actor.clone()))
            .app_data(Data::new(db_manager))
            .default_service(web::route().to(not_found::handle))
            .configure(auth::config)
    }).bind((server_config.host(), server_config.port()))?
        .run()
        .await
}