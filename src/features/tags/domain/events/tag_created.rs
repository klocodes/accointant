use serde::{Deserialize, Serialize};
use crate::support::id::Id;

pub const TAG_CREATED_NAME: &str = "tag_created";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCreated {
    id: Id,
    name: String,
    payload: TagCreatedPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCreatedPayload {
    id: Id,
    user_id: Id,
    name: String,
}

impl TagCreated {
    pub fn new(id: Id, tag_id: Id, user_id: Id, tag_name: String) -> Self {
        Self {
            id,
            name: TAG_CREATED_NAME.to_string(),
            payload: TagCreatedPayload {
                id: tag_id,
                user_id,
                name: tag_name,
            }
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &TagCreatedPayload {
        &self.payload
    }
}

impl TagCreatedPayload {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}