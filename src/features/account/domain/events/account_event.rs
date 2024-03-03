use serde::{Deserialize, Serialize};
use crate::features::account::domain::events::account_created::AccountCreated;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountEvent {
    AccountCreated(AccountCreated),
}

impl AccountEvent {
    pub fn name(&self) -> &str {
        match self {
            Self::AccountCreated(event) => event.name(),
        }
    }
}