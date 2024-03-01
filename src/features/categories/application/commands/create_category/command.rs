use uuid::Uuid;
use crate::support::command_bus::Command;

#[derive(Debug)]
pub struct CreateCategoryCommand {
    user_id: Uuid,
    name: String,
    icon: Option<String>,
}

impl CreateCategoryCommand {
    pub fn new(user_id: Uuid, name: String, icon: Option<String>) -> Self {
        Self {
            user_id,
            name,
            icon,
        }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn category_name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> &Option<String> {
        &self.icon
    }
}

impl Command for CreateCategoryCommand {
    fn name() -> &'static str {
        "create_category"
    }
}