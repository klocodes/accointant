use serde::{Deserialize, Serialize};
use crate::features::account::domain::operation_kind::OperationKind;
use crate::features::account::domain::rate::Rate;
use crate::support::id::Id;

const NAME: &str = "account_operation_applied";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOperationApplied {
    id: Id,
    name: String,
    payload: AccountOperationAppliedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOperationAppliedPayload {
    account_id: Id,
    new_amount: f64,
    new_currency_amount: f64,
    current_rate: Rate,
    operation_kind: OperationKind,
}

impl AccountOperationApplied {
    pub fn new(
        id: Id,
        account_id: Id,
        new_amount: f64,
        new_currency_amount: f64,
        current_rate: Rate,
        operation_kind: OperationKind,
    ) -> Self {
        Self {
            id,
            name: NAME.to_string(),
            payload: AccountOperationAppliedPayload {
                account_id,
                new_amount,
                new_currency_amount,
                current_rate,
                operation_kind,
            },
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &AccountOperationAppliedPayload {
        &self.payload
    }
}

impl AccountOperationAppliedPayload {
    pub fn account_id(&self) -> Id {
        self.account_id.clone()
    }

    pub fn new_amount(&self) -> f64 {
        self.new_amount
    }

    pub fn new_currency_amount(&self) -> f64 {
        self.new_currency_amount
    }

    pub fn current_rate(&self) -> Rate {
        self.current_rate.clone()
    }

    pub fn operation_kind(&self) -> OperationKind {
        self.operation_kind.clone()
    }
}