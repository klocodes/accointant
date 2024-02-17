use serde::{Deserialize, Serialize};
use crate::features::shared::id::Id;

pub const CATEGORY_CREATED_NAME: &str = "category_created";

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoryCreated {
    id: Id,
    name: String,
    payload: CategoryCreatedPayload,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoryCreatedPayload {
    id: Id,
    user_id: Id,
    name: String,
    icon: Option<String>,
}

impl CategoryCreated {
    pub fn new(id: Id, category_id: Id, user_id: Id, name: String, icon: Option<String>) -> Self {
        Self {
            id,
            name: CATEGORY_CREATED_NAME.to_string(),
            payload: CategoryCreatedPayload {
                id: category_id,
                user_id,
                name,
                icon,
            },
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &CategoryCreatedPayload {
        &self.payload
    }
}

impl CategoryCreatedPayload {
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