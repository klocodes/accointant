use serde::{Deserialize, Serialize};
use crate::features::balance::domain::events::balance_changed::BalanceChanged;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BalanceEvent {
    BalanceChanged(BalanceChanged),
}

impl BalanceEvent {
    pub fn new(balance: BalanceChanged) -> Self {
        Self::BalanceChanged(balance)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::BalanceChanged(balance) => balance.name(),
        }
    }

    pub fn payload(&self) -> &BalanceChanged {
        match self {
            Self::BalanceChanged(balance) => balance,
        }
    }
}