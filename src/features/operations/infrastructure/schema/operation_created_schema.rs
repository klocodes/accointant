use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::operations::domain::operation_event::{OperationCreatedEventData, OperationEvent};
use crate::support::data_mapper::DataMapper;

#[derive(Clone, Serialize, Deserialize)]
pub struct OperationCreatedEventSchema {
    id: Uuid,
    name: String,
    operation_id: Uuid,
    user_id: Uuid,
    kind: String,
    category_id: Uuid,
    amount: f64,
    amount_currency: f64,
    currency: String,
    rate: f64,
    label: String,
    tags: Vec<Uuid>,
    created_at: DateTime<Utc>,
}

impl DataMapper for OperationCreatedEventSchema {
    type Schema = OperationCreatedEventSchema;

    type Entity = OperationCreatedEventData;
}

impl OperationCreatedEventSchema {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn operation_id(&self) -> &Uuid {
        &self.operation_id
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn category_id(&self) -> &Uuid {
        &self.category_id
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    pub fn amount_currency(&self) -> &f64 {
        &self.amount_currency
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn rate(&self) -> &f64 {
        &self.rate
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn tags(&self) -> &Vec<Uuid> {
        &self.tags
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}