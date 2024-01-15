use actix::Actor;
use crate::config::actor::ConfigActor;
use crate::db::connection::connect;
use crate::db::data_mapper::DataMapper;

mod config;
mod errors;
mod features;
mod http;
mod log;
mod db;
mod service;


#[actix_web::main]
async fn main() {
    let config = config::Config::new();
    let log_config = config.log();
    let db_config = config.db().pg();
    let server_config = config.server();

    let config_actor = ConfigActor::new(config.clone()).start();

    let _guard = log::logger::init(log_config).await.unwrap();

    let pool = connect(db_config).await;

    http::server::run(server_config, config_actor, pool).await.expect("Failed to start server");

    std::mem::forget(_guard);
}
