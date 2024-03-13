use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::di::service_container::ServiceContainer;
use crate::events::event_bus::EventBus;
use crate::features::account::application::commands::account_creation::command::CreateAccountCommand;
use crate::features::account::application::commands::account_creation::handler::CreateAccountCommandHandler;
use crate::features::account::infrastructure::db_account_repository::DbAccountRepository;
use crate::http::error::HttpError;
use crate::http::extractors::jwt::Jwt;
use crate::services::jwt::JwtService;

#[derive(Debug, Clone, Deserialize)]
pub struct RequestData {
    name: String,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
    icon: String,
    source: Option<String>,
}

impl RequestData {
    fn into_command(self, user_id: Uuid) -> CreateAccountCommand {
        CreateAccountCommand::new(
            user_id,
            self.name,
            self.amount,
            self.currency,
            self.currency_amount,
            self.rate,
            self.icon,
            self.source
        )
    }

}

#[post("/create")]
pub async fn create_account(
    jwt: Jwt,
    request_data: Json<RequestData>,
    service_container: Data<Arc<ServiceContainer>>,
    event_bus: Data<Arc<Mutex<Box<dyn EventBus>>>>
) -> Result<impl Responder, HttpError> {
    let mut event_bus = event_bus.lock().await;
    let jwt_service = service_container.jwt_service();
    let claims = jwt_service.verify(jwt.0.as_str()).map_err(|e|
        HttpError::Service(e.to_string())
    )?;
    let user_id = claims.user_id().map_err(|e|
        HttpError::Service(e.to_string())
    )?;

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let rep = DbAccountRepository::new(db_manager.clone(), serializer);

    let handler = CreateAccountCommandHandler::new(rep);
    let command = request_data.clone().into_command(user_id);

    let mut command_bus = service_container.command_bus();
    command_bus.register(handler);
    let events = command_bus.dispatch(command).await.map_err(|e|
        HttpError::Service(e.to_string())
    )?;

    for event in events {
        event_bus.publish(event).await.map_err(|e|
            HttpError::Service(e.to_string())
        )?;
    }

    Ok(
        HttpResponse::Ok().finish()
    )
}