use serde::{Deserialize, Serialize};
use crate::features::balance::domain::events::balance_event::BalanceEvent;
use crate::features::categories::domain::events::category_event::CategoryEvent;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::features::tags::domain::events::tag_event::TagEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    OperationEvent(OperationEvent),
    CategoryEvent(CategoryEvent),
    TagEvent(TagEvent),
    BalanceEvent(BalanceEvent)
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Event::OperationEvent(operation_event) => operation_event.name(),
            Event::CategoryEvent(category_event) => category_event.name(),
            Event::TagEvent(tag_event) => tag_event.name(),
            Event::BalanceEvent(balance_event) => balance_event.name(),
        }
    }
}