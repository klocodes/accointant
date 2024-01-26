use actix::Actor;

use crate::config::manager::ConfigManager;
use crate::di::service_container::ServiceContainer;
use crate::http::server;
use crate::log::logger;

mod config;
mod errors;
mod features;
mod http;
mod log;
mod db;
mod services;
mod di;


#[actix_web::main]
async fn main() {
    let service_container = ServiceContainer::new(ConfigManager::new()).await.expect("Failed to create service container");

    let _guard = logger::init(service_container.config().log().clone());

    server::run(service_container)
        .await
        .expect("Failed to start server");

    std::mem::forget(_guard);
}
