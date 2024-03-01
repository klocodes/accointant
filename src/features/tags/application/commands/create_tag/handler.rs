use crate::features::tags::domain::events::tag_created::TAG_CREATED_NAME;
use async_trait::async_trait;
use crate::events::event::Event;
use crate::features::tags::application::commands::create_tag::command::CreateTagCommand;
use crate::features::tags::domain::error::DomainError;
use crate::features::tags::domain::tag::Tag;
use crate::features::tags::domain::tag_repository::TagRepository;
use crate::features::tags::domain::events::tag_deleted::TAG_DELETED_NAME;
use crate::features::tags::domain::events::tag_event::TagEvent;
use crate::features::tags::error::TagError;
use crate::support::command_bus::{Command, CommandHandler};
use crate::support::error::FeatureError;

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
    async fn handle(&mut self, command: CreateTagCommand) -> Result<Vec<Event>, FeatureError> {
        let exists = self.tag_repository.exists(TAG_CREATED_NAME, TAG_DELETED_NAME, command.tag_name())
            .await
            .map_err(|e|
                FeatureError::Tag(e)
            )?;

        if exists {
            return Err(
                FeatureError::Tag(
                    TagError::Domain(
                        DomainError::TagAlreadyExists(command.tag_name().to_string())
                    )
                )
            );
        }

        let event = Tag::handle_creation(command)
            .map_err(|e|
                FeatureError::Tag(
                    TagError::Domain(e)
                )
            )?;

        match event.clone() {
            TagEvent::TagCreated(tag_created) => {
                self.tag_repository.persist_tag_created_event(&tag_created)
                    .await
                    .map_err(|e|
                        FeatureError::Tag(e)
                    )?;
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
    use crate::features::tags::domain::tag_repository::MockTagRepository;
    use crate::features::tags::infrastructure::error::InfrastructureError;
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

        println!("{:?}", result);
        assert!(
            matches!(result, Err(
                FeatureError::Tag(
                    TagError::Infrastructure(
                        InfrastructureError::Repository(..)
                    )
                )
            ))
        );
    }

    #[tokio::test]
    async fn test_create_tag_command_handler_repository_error() {
        let rep = MockTagRepository::new(false, true);

        let mut create_tag_command_handler = CreateTagCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_tag_command_handler.handle(command).await;

        assert!(
            matches!(
                result,
                Err(
                    FeatureError::Tag(
                        TagError::Infrastructure(..)
                    )
                )
            )
        );
    }


    fn create_command_fixture() -> CreateTagCommand {
        CreateTagCommand::new(
            Uuid::new_v4(),
            "Test Tag".to_string(),
        )
    }
}