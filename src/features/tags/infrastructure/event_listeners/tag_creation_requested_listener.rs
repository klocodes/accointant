use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::events::event::Event;
use crate::events::event_listener::EventListener;
use crate::features::tags::application::commands::create_tag::command::CreateTagCommand;
use crate::features::tags::application::commands::create_tag::handler::CreateTagCommandHandler;
use crate::features::tags::domain::tag_repository::TagRepository;
use crate::features::operations::domain::events::tag_creation_requested::TagCreationRequested;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::support::command_bus::{CommandBus};

const EVENT_NAME: &str = "tag_creation_requested";

pub struct TagCreationRequestedListener<R>
    where
        R: TagRepository + Clone + Send + Sync + 'static,
{
    command_bus: Arc<Mutex<CommandBus<CreateTagCommand, CreateTagCommandHandler<R>>>>,
}

#[async_trait]
impl<R> EventListener for TagCreationRequestedListener<R>
    where
        R: TagRepository + Clone + Send + Sync + 'static,
{
    async fn on_event(&mut self, event: Event) -> Result<Vec<Event>, Error> {
        let event = self.parse_event(event)?;

        let command = CreateTagCommand::new(event.payload().user_id().value(), event.payload().tag_name().to_string());

        let mut guard = self.command_bus.lock().await;
        let events = guard.dispatch(command).await?;

        Ok(events)
    }

    fn event_name(&self) -> &str {
        EVENT_NAME
    }
}

impl<R> TagCreationRequestedListener<R>
    where
        R: TagRepository + Clone + Send + Sync + 'static,
{
    pub async fn new(
        command_bus: Arc<Mutex<CommandBus<CreateTagCommand, CreateTagCommandHandler<R>>>>,
        rep: R
    ) -> Self {
        let handler = CreateTagCommandHandler::new(rep);

        let mut guard = command_bus.lock().await;
        guard.register(handler.clone());

        Self {
            command_bus: command_bus.clone(),
        }
    }

    pub fn parse_event(&self, event: Event) -> Result<TagCreationRequested, Error> {
        match event {
            Event::OperationEvent(operation_event) => {
                match operation_event {
                    OperationEvent::TagCreationRequested(tag_creation_requested) => {
                        Ok(tag_creation_requested)
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