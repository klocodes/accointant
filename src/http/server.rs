use actix::Addr;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::config::actor::ConfigActor;
use crate::config::server::ServerConfig;
use crate::db::connection::DbPool;
use crate::http::handlers::auth;
use crate::http::handlers::errors::not_found;
use crate::http::middleware::error_handling::ErrorHandling;

pub async fn run(server_config: &ServerConfig, config_actor: Addr<ConfigActor>, db_connection_pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            //.wrap(ErrorHandling)
            .app_data(Data::new(config_actor.clone()))
            .app_data(Data::new(db_connection_pool.clone()))
            .default_service(web::route().to(not_found::handle))
            .configure(auth::config)
    }).bind((server_config.host(), server_config.port()))?
        .run()
        .await
}