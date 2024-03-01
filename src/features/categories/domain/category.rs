use crate::features::categories::application::commands::create_category::command::CreateCategoryCommand;
use crate::features::categories::domain::error::DomainError;
use crate::features::categories::domain::events::category_created::CategoryCreated;
use crate::features::categories::domain::events::category_event::CategoryEvent;
use crate::support::id::Id;

pub struct Category {
    id: Id,
    user_id: Id,
    name: String,
    icon: Option<String>,
}

impl Category {
    pub fn handle_creation(command: CreateCategoryCommand) -> Result<CategoryEvent, DomainError> {
        let id = Id::new(Id::generate());
        let user_id = Id::new(command.user_id().clone());
        let name = command.category_name().to_string();
        let icon = command.icon().clone();

        let category = Self {
            id,
            user_id,
            name,
            icon,
        };

        let category_created = CategoryEvent::CategoryCreated(
            CategoryCreated::new(
                Id::new(Id::generate()),
                category.id().clone(),
                category.user_id().clone(),
                category.name().to_string(),
                category.icon().clone(),
            )
        );

        Ok(category_created)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> &Option<String> {
        &self.icon
    }
}