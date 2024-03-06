use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::events::error::EventError;
use crate::events::event::Event;
use crate::events::event_listener::EventListener;
use crate::features::account::application::commands::apply_operation::command::ApplyOperationCommand;
use crate::features::account::application::commands::apply_operation::handler::ApplyOperationCommandHandler;
use crate::features::account::domain::account_repository::AccountRepository;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::services::serializer::Serializer;
use crate::support::command_bus::CommandBus;

pub struct OperationCreatedListener<R>
    where
        R: AccountRepository + Send + Sync + 'static,
{
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
    command_bus: CommandBus<ApplyOperationCommand, ApplyOperationCommandHandler<R>>,
}

#[async_trait]
impl<R> EventListener for OperationCreatedListener<R>
    where
        R: AccountRepository + Send + Sync + 'static,
{
    async fn on_event(&mut self, event: Event) -> Result<Vec<Event>, EventError> {
        let operation_created = self.parse_event(event)?;

        let command = ApplyOperationCommand::new(
            operation_created.payload().account_id().value(),
            operation_created.payload().operation_id().value(),
            operation_created.payload().amount().value(),
            operation_created.payload().currency().to_str().to_string(),
        );

        let events = self.command_bus.dispatch(command)
            .await
            .map_err(|e| EventError::Feature(e))?;

        Ok(events)
    }

    fn event_name(&self) -> &str {
        "OperationCreated"
    }
}

impl<R> OperationCreatedListener<R>
    where
        R: AccountRepository + Send + Sync + 'static,
{
    pub fn new(
        db_manager: Arc<Mutex<DbManager>>,
        serializer: Serializer,
        command_bus: CommandBus<ApplyOperationCommand, ApplyOperationCommandHandler<R>>,
    ) -> Self {
        Self {
            db_manager,
            serializer,
            command_bus,
        }
    }

    pub fn parse_event(&self, event: Event) -> Result<OperationCreated, EventError> {
        match event {
            Event::OperationEvent(operation_event) => match operation_event {
                OperationEvent::OperationCreated(operation_created) => Ok(operation_created),
                _ => Err(
                    EventError::Parsing(
                        "OperationCreatedListener: cannot parse OperationEvent".to_string()
                    )
                ),
            },
            _ => Err(
                EventError::Parsing(
                    "OperationCreatedListener: cannot parse OperationEvent".to_string()
                )
            ),
        }
    }
}