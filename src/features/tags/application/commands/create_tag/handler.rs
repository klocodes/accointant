use crate::features::tags::domain::events::tag_created::TAG_CREATED_NAME;
use async_trait::async_trait;
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::events::event::Event;
use crate::features::tags::application::commands::create_tag::command::CreateTagCommand;
use crate::features::tags::domain::tag::Tag;
use crate::features::tags::domain::tag_repository::TagRepository;
use crate::features::tags::domain::events::tag_deleted::TAG_DELETED_NAME;
use crate::features::tags::domain::events::tag_event::TagEvent;
use crate::support::command_bus::{Command, CommandHandler};

#[derive(Debug, Clone)]
pub struct CreateTagCommandHandler<R>
    where
        R: TagRepository + Send + Sync,
{
    tag_repository: R,
}

impl<R> CreateTagCommandHandler<R>
    where
        R: TagRepository + Send + Sync,
{
    pub fn new(tag_repository: R) -> Self {
        Self {
            tag_repository,
        }
    }
}

#[async_trait]
impl<R> CommandHandler<CreateTagCommand> for CreateTagCommandHandler<R>
    where
        R: TagRepository + Send + Sync,
{
    async fn handle(&mut self, command: CreateTagCommand) -> Result<Vec<Event>, Error> {
        let exists = self.tag_repository.exists(TAG_CREATED_NAME, TAG_DELETED_NAME, command.tag_name()).await?;

        if exists {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some(
                            format!("Tag with name {} already exists", CreateTagCommand::name()).into()
                        )
                    }
                )
            );
        }

        let event = Tag::handle_creation(command)?;

        match event.clone() {
            TagEvent::TagCreated(tag_created) => {
                self.tag_repository.persist_tag_created_event(&tag_created).await?;
            }
        };

        Ok(
            vec![Event::TagEvent(event)]
        )
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::errors::server::ServerErrors::InternalServerError;
    use crate::features::tags::domain::tag_repository::MockTagRepository;
    use super::*;

    #[tokio::test]
    async fn test_create_tag_command_handler_success() {
        let rep = MockTagRepository::new(false, false);

       let mut create_tag_command_handler = CreateTagCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_tag_command_handler.handle(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_tag_command_handler_existing_tag() {
        let rep = MockTagRepository::new(true, false);

        let mut create_tag_command_handler = CreateTagCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_tag_command_handler.handle(command).await;

        assert!(matches!(result, Err(Error::Client(BadRequest { .. }))));
    }

    #[tokio::test]
    async fn test_create_tag_command_handler_repository_error() {
        let rep = MockTagRepository::new(false, true);

        let mut create_tag_command_handler = CreateTagCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_tag_command_handler.handle(command).await;

        assert!(matches!(result, Err(Error::Server(InternalServerError { .. }))));
    }



    fn create_command_fixture() -> CreateTagCommand {
        CreateTagCommand::new(
            Uuid::new_v4(),
            "Test Tag".to_string(),
        )
    }
}