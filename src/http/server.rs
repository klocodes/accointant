use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use crate::bootstrap::app_context::AppContext;
use crate::config::server::ServerConfig;
use crate::http::handlers::auth;
use crate::http::handlers::errors::not_found;

pub async fn run(server_config: &ServerConfig, app_context: AppContext) -> std::io::Result<()>
{
    HttpServer::new(move || {
        App::new()
            //.wrap(ErrorHandling)
            .app_data(Data::new(app_context.clone()))
            .default_service(web::route().to(not_found::handle))
            .configure(auth::config)
    }).bind((server_config.host(), server_config.port()))?
        .run()
        .await
}