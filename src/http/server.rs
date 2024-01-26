use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::di::service_container::ServiceContainer;
use crate::http::routes::Routes;

pub async fn run(service_container: ServiceContainer) -> std::io::Result<()> {
    let cfg = service_container.config().server().clone();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service_container.clone()))
            .configure(Routes::new)
    }).bind((cfg.host(), cfg.port()))?
        .run()
        .await
}