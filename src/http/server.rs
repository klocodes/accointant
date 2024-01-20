use std::sync::{Arc};
use actix::Addr;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use tokio::sync::Mutex;

use crate::config::actor::ConfigActor;
use crate::config::server::ServerConfig;
use crate::db::manager::db_manager::DbManager;
use crate::http::handlers::auth;
use crate::http::handlers::errors::not_found;

pub async fn run<M>(server_config: &ServerConfig, config_actor: Addr<ConfigActor>, db_manager: M) -> std::io::Result<()>
    where M: DbManager + 'static + Send,
{
    HttpServer::new(move || {
        App::new()
            //.wrap(ErrorHandling)
            .app_data(Data::new(config_actor.clone()))
            .app_data(Data::new(db_manager.clone()))
            .default_service(web::route().to(not_found::handle))
            .configure(auth::config)
    }).bind((server_config.host(), server_config.port()))?
        .run()
        .await
}