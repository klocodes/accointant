use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::infrastructure::error::InfrastructureError;
use crate::support::data_mapper::DataMapper;

#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct AccountEventSchema {
    id: Uuid,
    name: String,
    payload: serde_json::Value,
}

impl AccountEventSchema {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> serde_json::Value {
        self.payload.clone()
    }

    pub fn decode_payload(&mut self) -> Result<(), InfrastructureError> {
        self.payload = serde_json::from_value(self.payload.clone()).map_err(|e|
            InfrastructureError::Repository(
                format!("Failed to decode account event payload. {}", e.to_string())
            )
        )?;

        Ok(())
    }
}

impl DataMapper for AccountEventSchema {
    type Schema = Self;
    type Entity = AccountEvent;
}