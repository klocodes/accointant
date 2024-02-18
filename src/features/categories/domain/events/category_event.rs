use serde::{Deserialize, Serialize};
use crate::features::categories::domain::events::category_created::CategoryCreated;

#[derive(Clone, Serialize, Deserialize)]
pub enum CategoryEvent {
    CategoryCreated(CategoryCreated),
}

impl CategoryEvent {
   pub fn name(&self) -> &str {
       match self {
           CategoryEvent::CategoryCreated(event) => event.name()
       }
   }
}