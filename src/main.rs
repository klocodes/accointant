use actix::Actor;
use crate::bootstrap::app_context::AppContext;

use crate::service::data_mapper::DataMapper;
use crate::db::db_manager::DbManager;

mod config;
mod errors;
mod feature;
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
