use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use crate::di::service_container::ServiceContainer;
use crate::events::event_bus::EventBus;
use crate::http::routes::Routes;

pub async fn run(
    service_container: Arc<ServiceContainer>,
    event_bus: Arc<Box<dyn EventBus>>,
) -> std::io::Result<()> {
    let cfg = service_container.config().server().clone();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus.clone()))
            .configure(Routes::new)
    }).bind((cfg.host(), cfg.port()))?
        .run()
        .await
}