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
pub mod test_utils;

// Private modules
pub mod features;
mod log;
pub mod support;

pub struct App;

impl App {
    pub async fn start() {
        dotenv().ok();

        let config = ConfigManager::new();

        let service_container = ServiceContainer::new(config).await.expect("Failed to create service container");
        let service_container = Arc::new(service_container);

        let _guard = logger::init(service_container.config().log().clone());

        let (event_bus, queue_receiver, mut response) = EventBusFactory::create(service_container.clone()).await.expect("Failed to create event bus");

        let event_bus_clone = event_bus.clone();
        tokio::spawn(async move {
            if let Err(e) = event_bus_clone.start(queue_receiver).await {
                log_error!("{}", e.to_string());
                log_trace!("{}", e.to_string());
            }

            while let responder = response.recv().await.expect("Failed to receive response") {
                let _ = responder.handle().await;
            }
        });

        server::run(service_container, event_bus)
            .await
            .expect("Failed to start server");

        std::mem::forget(_guard);
    }
}