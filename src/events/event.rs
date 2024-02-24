use serde::{Deserialize, Serialize};
use crate::features::categories::domain::events::category_event::CategoryEvent;
use crate::features::operations::domain::events::operation_event::OperationEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    OperationEvent(OperationEvent),
    CategoryEvent(CategoryEvent),
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Event::OperationEvent(operation_event) => operation_event.name(),
            Event::CategoryEvent(category_event) => category_event.name(),
        }
    }
}