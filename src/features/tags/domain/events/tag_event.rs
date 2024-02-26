use serde::{Deserialize, Serialize};
use crate::features::tags::domain::events::tag_created::TagCreated;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TagEvent {
    TagCreated(TagCreated),
}

impl TagEvent {
    pub fn name(&self) -> &str {
        match self {
            Self::TagCreated(event) => event.name()
        }
    }
}