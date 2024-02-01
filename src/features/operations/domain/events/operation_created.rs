use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::features::operations::domain::amount::Amount;
use crate::features::operations::domain::currency::Currency;
use crate::features::operations::domain::kind::Kind;
use crate::features::shared::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationCreated {
    id: Id,
    name: String,
    operation_id: Id,
    user_id: Id,
    kind: Kind,
    category_id: Id,
    amount: Amount,
    amount_currency: Amount,
    currency: Currency,
    rate: Amount,
    label: String,
    tag_ids: Vec<Id>,
    created_at: DateTime<Utc>,
}

impl OperationCreated {
    pub fn new(
        id: Id,
        operation_id: Id,
        user_id: Id,
        kind: Kind,
        category_id: Id,
        amount: Amount,
        amount_currency: Amount,
        currency: Currency,
        rate: Amount,
        label: String,
        tag_ids: Vec<Id>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name: "operation_created".to_string(),
            operation_id,
            user_id,
            kind,
            category_id,
            amount,
            amount_currency,
            currency,
            rate,
            label,
            tag_ids,
            created_at,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn operation_id(&self) -> &Id {
        &self.operation_id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn category_id(&self) -> &Id {
        &self.category_id
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn currency_amount(&self) -> &Amount {
        &self.amount_currency
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn rate(&self) -> &Amount {
        &self.rate
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn tag_ids(&self) -> &Vec<Id> {
        &self.tag_ids
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}