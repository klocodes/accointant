use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::events::event_bus::EventBus;
use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::application::commands::create_category::handler::CreateCategoryCommandHandler;
use crate::features::categories::infrastructure::db_category_repository::DbCategoryRepository;
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
    event_bus: Data<Arc<Box<dyn EventBus>>>
) -> Result<impl Responder, Error> {
    let service_container = service_container.into_inner().clone();
    let event_bus = event_bus.into_inner().as_ref().clone();

    let jwt_service = service_container.jwt_service();
    let claims = jwt_service.verify(jwt.0.as_str())?;
    let user_id = claims.user_id()?;

    let db_manager = service_container.db_manager();
    let rep = DbCategoryRepository::new(db_manager.clone(), service_container.serializer());

    let command = CreateCategoryCommand::new(user_id, request_data.name.clone(), request_data.icon.clone());
    let handler = CreateCategoryCommandHandler::new(rep);

    let mut command_bus = service_container.command_bus();
    command_bus.register(handler);
    let events = command_bus.dispatch(command).await?;

    for event in events {
        event_bus.publish(event).await?;
    }

    Ok(
        HttpResponse::Ok()
    )
}