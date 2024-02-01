use serde::{Deserialize, Serialize};
use crate::features::shared::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCreationRequested {
    id: Id,
    name: String,
    operation_id: Id,
    user_id: Id,
    category_id: Id,
    category_name: String,
}

impl CategoryCreationRequested {
    pub fn new(
        id: Id,
        operation_id: Id,
        user_id: Id,
        category_id: Id,
        category_name: String,
    ) -> Self {
        Self {
            id,
            name: "category_creation_requested".to_string(),
            operation_id,
            user_id,
            category_id,
            category_name,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn operation_id(&self) -> &Id {
        &self.operation_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn category_id(&self) -> &Id {
        &self.category_id
    }

    pub fn category_name(&self) -> &String {
        &self.category_name
    }
}