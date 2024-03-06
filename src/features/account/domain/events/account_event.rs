use serde::{Deserialize, Serialize};
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::domain::events::account_operation_applied::AccountOperationApplied;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountEvent {
    AccountCreated(AccountCreated),
    AccountOperationApplied(AccountOperationApplied),
}

impl AccountEvent {
    pub fn name(&self) -> &str {
        match self {
            Self::AccountCreated(event) => event.name(),
            Self::AccountOperationApplied(event) => event.name(),
        }
    }
}