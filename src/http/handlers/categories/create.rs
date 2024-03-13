use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::di::service_container::ServiceContainer;
use crate::events::event_bus::EventBus;
use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::application::commands::create_category::handler::CreateCategoryCommandHandler;
use crate::features::categories::infrastructure::db_category_repository::DbCategoryRepository;
use crate::http::error::HttpError;
use crate::http::extractors::jwt::Jwt;
use crate::services::jwt::JwtService;

#[derive(Deserialize)]
struct RequestData {
    name: String,
    icon: Option<String>
}

#[post("/create")]
pub async fn create_category(
    request_data: Json<RequestData>,
    jwt: Jwt,
    service_container: Data<Arc<ServiceContainer>>,
    event_bus: Data<Arc<Mutex<Box<dyn EventBus>>>>
) -> Result<impl Responder, HttpError> {
    let service_container = service_container.into_inner().clone();
    let mut event_bus = event_bus.lock().await;

    let jwt_service = service_container.jwt_service();
    let claims = jwt_service.verify(jwt.0.as_str()).map_err(|e|
        HttpError::Service(e.to_string())
    )?;
    let user_id = claims.user_id().map_err(|e|
        HttpError::Service(e.to_string())
    )?;

    let db_manager = service_container.db_manager();
    let rep = DbCategoryRepository::new(db_manager.clone(), service_container.serializer());

    let command = CreateCategoryCommand::new(user_id, request_data.name.clone(), request_data.icon.clone());
    let handler = CreateCategoryCommandHandler::new(rep);

    let mut command_bus = service_container.command_bus();
    command_bus.register(handler);
    let events = command_bus.dispatch(command)
        .await
        .map_err(|e|
            HttpError::Feature(e)
        )?;

    for event in events {
        event_bus.publish(event)
            .await
            .map_err(|e|
                HttpError::Event(e)
            )?;
    }

    Ok(
        HttpResponse::Ok()
    )
}