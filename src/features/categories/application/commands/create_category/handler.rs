use async_trait::async_trait;
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::events::event::Event;
use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::domain::category::Category;
use crate::features::categories::domain::category_repository::CategoryRepository;
use crate::features::categories::domain::events::category_created::CATEGORY_CREATED_NAME;
use crate::features::categories::domain::events::category_deleted::CATEGORY_DELETED_NAME;
use crate::features::categories::domain::events::category_event::CategoryEvent;
use crate::support::command_bus::CommandHandler;

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
    async fn handle(&mut self, command: CreateCategoryCommand) -> Result<Vec<Event>, Error> {
        let exists = self.category_repository.exists(CATEGORY_CREATED_NAME, CATEGORY_DELETED_NAME, &command.name()).await?;

        if exists {
            return Err(
                Error::Client(
                    BadRequest {
                        message: Some(
                            format!("Category with name {} already exists", command.name()).into()
                        )
                    }
                )
            );
        }

        let event = Category::handle_creation(command)?;

        match event.clone() {
            CategoryEvent::CategoryCreated(category_created) => {
                self.category_repository.persist_category_created_event(&category_created).await?;
            }
        };

        Ok(
            vec![Event::CategoryEvent(event)]
        )
    }
}