use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::features::flow::domain::amount::Amount;
use crate::features::flow::domain::currency::Currency;
use crate::features::flow::domain::kind::Kind;
use crate::features::shared::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationEvent {
    OperationCreated(OperationCreatedEventData),
    CategoryCreationRequested(CategoryCreationRequestedEventData),
    TagCreationRequested(TagCreationRequestedEventData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationCreatedEventData {
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
}

impl OperationCreatedEventData {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCreationRequestedEventData {
    id: Id,
    user_id: Id,
    category_id: Id,
    category_name: String,
}

impl CategoryCreationRequestedEventData {
    pub fn new(
        id: Id,
        user_id: Id,
        category_id: Id,
        category_name: String,
    ) -> Self {
        Self {
            id,
            user_id,
            category_id,
            category_name,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn category_id(&self) -> &Id {
        &self.category_id
    }

    pub fn category_name(&self) -> &String {
        &self.category_name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCreationRequestedEventData {
    id: Id,
    user_id: Id,
    tag_id: Id,
    tag_name: String,
}

impl TagCreationRequestedEventData {
    pub fn new(
        id: Id,
        user_id: Id,
        tag_id: Id,
        tag_name: String,
    ) -> Self {
        Self {
            id,
            user_id,
            tag_id,
            tag_name,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn tag_id(&self) -> &Id {
        &self.tag_id
    }

    pub fn tag_name(&self) -> &String {
        &self.tag_name
    }
}