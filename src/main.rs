use std::sync::{Arc, Mutex};
use actix::Actor;
use crate::bootstrap::app_context;
use crate::bootstrap::app_context::AppContext;

use crate::config::actor::ConfigActor;
use crate::service::data_mapper::DataMapper;
use crate::db::manager::db_manager::DbManager;
use crate::db::manager::factory::DbManagerFactory;

mod config;
mod errors;
mod features;
mod http;
mod log;
mod db;
mod service;
mod bootstrap;


#[actix_web::main]
async fn main() {
   let (app_context, _guard) = AppContext::new().await.expect("Failed to create app context");
   let server_config = app_context.get_config().server();

   http::server::run(server_config, app_context.clone()).await.expect("Failed to start server");

   std::mem::forget(_guard);
}
