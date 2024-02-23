use serde::{Deserialize, Serialize};
use crate::features::shared::id::Id;

pub const NAME: &str = "category_creation_requested";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCreationRequested {
    id: Id,
    name: String,
    payload: CategoryCreationRequestedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCreationRequestedPayload {
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
            name: NAME.to_string(),
            payload: CategoryCreationRequestedPayload {
                operation_id,
                user_id,
                category_id,
                category_name,
            },
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &CategoryCreationRequestedPayload {
        &self.payload
    }
}

impl CategoryCreationRequestedPayload {
    pub fn operation_id(&self) -> &Id {
        &self.operation_id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn category_id(&self) -> &Id {
        &self.category_id
    }

    pub fn category_name(&self) -> &str {
        &self.category_name
    }
}