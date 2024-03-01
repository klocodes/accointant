use serde::{Deserialize, Serialize};
use crate::support::id::Id;

pub const NAME: &str = "tag_creation_requested";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCreationRequested {
    id: Id,
    name: String,
    payload: TagCreationRequestedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCreationRequestedPayload {
    operation_id: Id,
    user_id: Id,
    tag_id: Id,
    tag_name: String,
}

impl TagCreationRequested {
    pub fn new(
        id: Id,
        operation_id: Id,
        user_id: Id,
        tag_id: Id,
        tag_name: String,
    ) -> Self {
        Self {
            id,
            name: NAME.to_string(),
            payload: TagCreationRequestedPayload {
                operation_id,
                user_id,
                tag_id,
                tag_name,
            },
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn payload(&self) -> &TagCreationRequestedPayload {
        &self.payload
    }
}

impl TagCreationRequestedPayload {
    pub fn operation_id(&self) -> &Id {
        &self.operation_id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn tag_id(&self) -> &Id {
        &self.tag_id
    }

    pub fn tag_name(&self) -> &String {
        &self.tag_name
    }
}