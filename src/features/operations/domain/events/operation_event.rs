use serde::{Deserialize, Serialize};
use crate::features::operations::domain::events::category_creation_requested::CategoryCreationRequested;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::domain::events::tag_creation_requested::TagCreationRequested;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OperationEvent {
    OperationCreated(OperationCreated),
    CategoryCreationRequested(CategoryCreationRequested),
    TagCreationRequested(TagCreationRequested)
}

impl OperationEvent {
    pub fn name(&self) -> &str {
        match self {
            OperationEvent::OperationCreated(_) => "operation_created",
            OperationEvent::CategoryCreationRequested(_) => "category_creation_requested",
            OperationEvent::TagCreationRequested(_) => "tag_creation_requested"
        }
    }
}