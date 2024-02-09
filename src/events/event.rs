use serde::{Deserialize, Serialize};
use crate::features::operations::domain::events::operation_event::OperationEvent;

#[derive(Clone, Serialize, Deserialize)]
pub enum Event {
    OperationEvent(OperationEvent),
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Event::OperationEvent(operation_event) => operation_event.name(),
        }
    }
}