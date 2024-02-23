use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::events::event::Event;
use crate::events::event_listener::EventListener;
use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::application::commands::create_category::handler::CreateCategoryCommandHandler;
use crate::features::categories::domain::category_repository::CategoryRepository;
use crate::features::operations::domain::events::category_creation_requested::CategoryCreationRequested;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::support::command_bus::{CommandBus};

const EVENT_NAME: &str = "category_creation_requested";

pub struct CategoryCreationRequestedListener<R>
    where
        R: CategoryRepository + Clone + Send + Sync + 'static,
{
    command_bus: Arc<Mutex<CommandBus<CreateCategoryCommand, CreateCategoryCommandHandler<R>>>>,
}

#[async_trait]
impl<R> EventListener for CategoryCreationRequestedListener<R>
    where
        R: CategoryRepository + Clone + Send + Sync + 'static,
{
    async fn on_event(&mut self, event: Event) -> Result<Vec<Event>, Error> {
        let event = self.parse_event(event)?;


        let command = CreateCategoryCommand::new(event.payload().user_id().value(), event.name().to_string(), None);

        let mut guard = self.command_bus.lock().await;
        let events = guard.dispatch(command).await?;

        Ok(events)
    }

    fn event_name(&self) -> &str {
        EVENT_NAME
    }
}

impl<R> CategoryCreationRequestedListener<R>
    where
        R: CategoryRepository + Clone + Send + Sync + 'static,
{
    pub async fn new(
        command_bus: Arc<Mutex<CommandBus<CreateCategoryCommand, CreateCategoryCommandHandler<R>>>>,
        rep: R
    ) -> Self {
        let handler = CreateCategoryCommandHandler::new(rep);

        let mut guard = command_bus.lock().await;
        guard.register(handler.clone());

        Self {
            command_bus: command_bus.clone(),
        }
    }

    pub fn parse_event(&self, event: Event) -> Result<CategoryCreationRequested, Error> {
        match event {
            Event::OperationEvent(operation_event) => {
                match operation_event {
                    OperationEvent::CategoryCreationRequested(category_creation_requested) => {
                        Ok(category_creation_requested)
                    }
                    _ => Err(
                        Error::Server(
                            InternalServerError {
                                context: Some("Invalid event type".into()),
                            }
                        )
                    )
                }
            }
            _ => Err(
                Error::Server(
                    InternalServerError {
                        context: Some("Invalid event type".into()),
                    }
                )
            )
        }
    }
}