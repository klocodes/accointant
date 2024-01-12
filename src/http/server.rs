use actix::Addr;
use actix_web::{App, HttpServer, web};

use crate::config::actor::ConfigActor;
use crate::config::server::ServerConfig;
use crate::http::handlers::auth;
use crate::http::handlers::errors::not_found;
use crate::http::middleware::error_handling::ErrorHandling;

pub async fn run(server_config: &ServerConfig, config_actor: Addr<ConfigActor>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandling)
            .app_data(config_actor.clone())
            .default_service(web::route().to(not_found::handle))
            .configure(auth::config)
    }).bind((server_config.host(), server_config.port()))?
        .run()
        .await
}