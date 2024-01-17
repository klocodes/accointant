use tokio::sync::Mutex;
use std::sync::Arc;
use actix::Actor;

use crate::config::actor::ConfigActor;
use crate::db::data_mapper::DataMapper;
use crate::db::manager::factory::DbFactory;

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
    let db_config = config.db();
    let server_config = config.server();

    let config_actor = ConfigActor::new(config.clone()).start();

    let _guard = log::logger::init(log_config).await.unwrap();

    let db_manager = DbFactory::create(&db_config).await.expect("Failed to create database manager");

    http::server::run(server_config, config_actor, db_manager).await.expect("Failed to start server");

    std::mem::forget(_guard);
}
