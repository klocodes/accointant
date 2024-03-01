use serde::{Deserialize, Serialize};
use crate::features::balance::domain::currency::Currency;
use crate::features::balance::domain::rate::Rate;
use crate::support::id::Id;

const NAME: &str = "balance_changed";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceChanged {
    id: Id,
    name: String,
    payload: BalanceChangedPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceChangedPayload {
    user_id: Id,
    amount: f64,
    currency: Currency,
    currency_amount: f64,
    rate: Rate,
}

impl BalanceChanged {
    pub fn new(id: Id, user_id: Id, amount: f64, currency: Currency, currency_amount: f64, rate: Rate) -> Self {
        Self {
            id,
            name: NAME.to_string(),
            payload: BalanceChangedPayload::new(user_id, amount, currency, currency_amount, rate),
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &BalanceChangedPayload {
        &self.payload
    }
}

impl BalanceChangedPayload {
    pub fn new(user_id: Id, amount: f64, currency: Currency, currency_amount: f64, rate: Rate) -> Self {
        Self { user_id, amount, currency, currency_amount, rate }
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn currency_amount(&self) -> f64 {
        self.currency_amount
    }

    pub fn rate(&self) -> &Rate {
        &self.rate
    }
}

