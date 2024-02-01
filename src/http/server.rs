use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::di::service_container::ServiceContainer;
use crate::events::event_bus::EventBus;
use crate::http::routes::Routes;

pub async fn run(
    service_container: Arc<ServiceContainer>,
    event_bus: Arc<impl EventBus>,
) -> std::io::Result<()> {
    let cfg = service_container.config().server().clone();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus.clone()))
            .configure(Routes::new)
    }).bind((cfg.host(), cfg.port()))?
        .run()
        .await
}