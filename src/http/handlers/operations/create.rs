use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use uuid::Uuid;
use crate::features::operations::infrastructure::db_operation_repository::DbOperationRepository;
use crate::di::service_container::ServiceContainer;
use crate::services::jwt::JwtService;
use crate::events::event_bus::EventBus;
use crate::features::operations::application::commands::create_operation::command::{CreateOperationCommand, TagData};
use crate::features::operations::application::commands::create_operation::handler::CreateOperationCommandHandler;
use crate::http::error::HttpError;
use crate::http::extractors::jwt::Jwt;

#[derive(serde::Deserialize)]
struct RequestData {
    kind: String,
    category_id: Option<Uuid>,
    category_name: String,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
    label: String,
    tags: Vec<RequestTagData>,
}

#[derive(serde::Deserialize)]
struct RequestTagData {
    id: Option<Uuid>,
    name: String,
}

impl RequestData {
    fn to_command(&self, user_id: Uuid) -> CreateOperationCommand {
        let tags = self.tags.iter().map(|tag| TagData::new(
            tag.id,
            tag.name.clone(),
        )).collect();

        CreateOperationCommand::new(
            self.kind.clone(),
            user_id,
            self.category_id,
            self.category_name.clone(),
            self.amount,
            self.currency.clone(),
            self.currency_amount,
            self.rate,
            self.label.clone(),
            tags,
        )
    }
}

#[post("/create")]
pub async fn create_operation(
    jwt: Jwt,
    request_data: Json<RequestData>,
    service_container: Data<Arc<ServiceContainer>>,
    event_bus: Data<Arc<Box<dyn EventBus>>>,
) -> Result<impl Responder, HttpError> {
    let jwt_service = service_container.jwt_service();
    let user_id = jwt_service.verify(jwt.0.as_str())
        .map_err(|e|
            HttpError::Service(e.to_string())
        )?
        .user_id()
        .map_err(|e|
            HttpError::Service(e.to_string())
        )?;

    let db_manager = service_container.db_manager();
    let rep = DbOperationRepository::new(db_manager, service_container.serializer());

    let command = request_data.to_command(user_id);
    let handler = CreateOperationCommandHandler::new(rep);

    let mut command_bus = service_container.command_bus();
    command_bus.register(handler);
    let events = command_bus.dispatch(command)
        .await
        .map_err(|e|
            HttpError::Feature(e)
        )?;

    for event in events {
        event_bus.publish(event).await
            .map_err(|e|
                HttpError::Event(e)
            )?;
    }

    Ok(HttpResponse::Ok())
}