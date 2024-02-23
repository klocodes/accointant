use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::support::command_bus::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOperationCommand {
    kind: String,
    user_id: Uuid,
    category_id: Option<Uuid>,
    category_name: String,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
    label: String,
    tags: Vec<TagData>,
}

impl Command for CreateOperationCommand {
    fn name() -> &'static str {
        "CreateOperationCommand"
    }
}

impl CreateOperationCommand {
    pub fn new(
        kind: String,
        user_id: Uuid,
        category_id: Option<Uuid>,
        category_name: String,
        amount: f64,
        currency: String,
        currency_amount: f64,
        rate: f64,
        label: String,
        tags: Vec<TagData>,
    ) -> Self {
        Self {
            kind,
            user_id,
            category_id,
            category_name,
            amount,
            currency,
            currency_amount,
            rate,
            label,
            tags,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn category_id(&self) -> &Option<Uuid> {
        &self.category_id
    }

    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn currency_amount(&self) -> f64 {
        self.currency_amount
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn tags(&self) -> &[TagData] {
        &self.tags
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    #[serde(rename = "tag_id")]
    id: Option<Uuid>,

    #[serde(rename = "tag_name")]
    name: String,
}

impl TagData {
    pub fn new (id: Option<Uuid>, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Option<Uuid> {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}