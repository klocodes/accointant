use async_trait::async_trait;
use crate::events::event::Event;
use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::domain::category::Category;
use crate::features::categories::domain::category_repository::CategoryRepository;
use crate::features::categories::domain::error::DomainError;
use crate::features::categories::domain::events::category_created::CATEGORY_CREATED_NAME;
use crate::features::categories::domain::events::category_deleted::CATEGORY_DELETED_NAME;
use crate::features::categories::domain::events::category_event::CategoryEvent;
use crate::features::categories::error::CategoryError;
use crate::support::command_bus::CommandHandler;
use crate::support::error::FeatureError;

#[derive(Debug, Clone)]
pub struct CreateCategoryCommandHandler<R>
    where
        R: CategoryRepository + Send + Sync,
{
    category_repository: R,
}

impl<R> CreateCategoryCommandHandler<R>
    where
        R: CategoryRepository + Send + Sync,
{
    pub fn new(category_repository: R) -> Self {
        Self {
            category_repository,
        }
    }
}

#[async_trait]
impl<R> CommandHandler<CreateCategoryCommand> for CreateCategoryCommandHandler<R>
    where
        R: CategoryRepository + Send + Sync,
{
    async fn handle(&mut self, command: CreateCategoryCommand) -> Result<Vec<Event>, FeatureError> {
        let exists = self.category_repository.exists(CATEGORY_CREATED_NAME, CATEGORY_DELETED_NAME, &command.category_name()).await
            .map_err(|e| FeatureError::Category(e))?;

        if exists {
            return Err(
                FeatureError::Category(
                    CategoryError::Domain(
                        DomainError::CategoryAlreadyExists(command.category_name().to_string())
                    )
                )
            );
        }

        let event = Category::handle_creation(command)
            .map_err(|e|
                FeatureError::Category(
                    CategoryError::Domain(e)
                )
            )?;

        match event.clone() {
            CategoryEvent::CategoryCreated(category_created) => {
                self.category_repository.persist_category_created_event(&category_created)
                    .await
                    .map_err(|e|
                        FeatureError::Category(e)
                    )?;
            }
        };

        Ok(
            vec![Event::CategoryEvent(event)]
        )
    }
}

#[cfg(test)]
mod tests {
    use futures_util::FutureExt;
    use uuid::Uuid;
    use crate::features::categories::domain::category_repository::MockCategoryRepository;
    use super::*;

    #[tokio::test]
    async fn test_create_category_command_handler_success() {
        let mut rep = MockCategoryRepository::new();
        rep.expect_exists()
            .times(1)
            .returning(|_, _, _| {
                async move {
                    Ok(false) // Симулируем асинхронное выполнение
                }.boxed()
            });
        rep.expect_persist_category_created_event()
            .times(1) // Ожидаем, что будет вызван ровно один раз
            .returning(|_| async { Ok(()) }.boxed());

        let mut create_category_command_handler = CreateCategoryCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_category_command_handler.handle(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_category_command_handler_existing_category() {
        let mut rep = MockCategoryRepository::new();
        rep.expect_exists()
            .times(1)
            .returning(|_, _, _| async { Ok(true) }.boxed()); // Имитация существующей категории

        let mut create_category_command_handler = CreateCategoryCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_category_command_handler.handle(command).await;

        assert!(matches!(result, Err(FeatureError::Category(..))));
    }

    #[tokio::test]
    async fn test_create_category_command_handler_repository_error() {
        let mut rep = MockCategoryRepository::new();
        rep.expect_exists()
            .times(1)
            .returning(|_, _, _| async {
                Err(
                    CategoryError::Domain(
                        DomainError::CategoryAlreadyExists("Test Category".to_string())
                    )
                )
            }.boxed());

        let mut create_category_command_handler = CreateCategoryCommandHandler::new(rep);

        let command = create_command_fixture();
        let result = create_category_command_handler.handle(command).await;

        assert!(matches!(result, Err(FeatureError::Category(..))));
    }


    fn create_command_fixture() -> CreateCategoryCommand {
        CreateCategoryCommand::new(
            Uuid::new_v4(),
            "Test Category".to_string(),
            None,
        )
    }
}