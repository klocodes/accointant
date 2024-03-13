use std::sync::Arc;
use dotenv::dotenv;

use crate::config::manager::ConfigManager;
use crate::di::service_container::ServiceContainer;
use crate::events::event_bus_factory::EventBusFactory;
use crate::http::server;
use crate::log::logger;

// Re-export for convenience in downstream crates (e.g. integration tests)
pub mod config;
pub mod db;
pub mod di;
pub mod events;
pub mod http;
pub mod mq;
pub mod services;
pub mod support;
pub mod test_utils;

// Private modules
mod features;
mod log;
mod sagas;

pub struct App;

impl App {
    pub async fn start() {
        dotenv().ok();

        let config = ConfigManager::new();

        let service_container = ServiceContainer::new(config).await.expect("Failed to create service container");
        let service_container = Arc::new(service_container);

        let _guard = logger::init(service_container.config().log().clone());

        let event_bus = EventBusFactory::create(service_container.clone()).await.expect("Failed to create event bus");

        server::run(service_container, event_bus)
            .await
            .expect("Failed to start server");

        std::mem::forget(_guard);
    }
}