use serde::{Deserialize, Serialize};
use crate::features::account::domain::currency::Currency;
use crate::features::account::domain::rate::Rate;
use crate::features::account::domain::source::Source;
use crate::support::id::Id;

pub const ACCOUNT_CREATED_NAME: &str = "account_created";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreated {
    id: Id,
    name: String,
    payload: AccountCreatedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreatedPayload {
    account_id: Id,
    user_id: Id,
    account_name: String,
    amount: f64,
    currency: Currency,
    currency_amount: f64,
    rate: Rate,
    icon: String,
    source: Option<Source>,
}

impl AccountCreated {
    pub fn new(
        id: Id,
        account_id: Id,
        user_id: Id,
        account_name: String,
        amount: f64,
        currency: Currency,
        currency_amount: f64,
        rate: Rate,
        icon: String,
        source: Option<Source>,
    ) -> Self {
        Self {
            id,
            name: ACCOUNT_CREATED_NAME.to_string(),
            payload: AccountCreatedPayload {
                account_id,
                user_id,
                account_name,
                amount,
                currency,
                currency_amount,
                rate,
                icon,
                source,
            }
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &AccountCreatedPayload {
        &self.payload
    }
}

impl AccountCreatedPayload {
    pub fn account_id(&self) -> &Id {
        &self.account_id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn currency_amount(&self) -> &f64 {
        &self.currency_amount
    }

    pub fn rate(&self) -> &Rate {
        &self.rate
    }

    pub fn icon(&self) -> &str {
        &self.icon
    }

    pub fn source(&self) -> &Option<Source> {
        &self.source
    }
}