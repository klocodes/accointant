use async_trait::async_trait;
use crate::errors::Error;
use crate::features::flow::domain::operation_event::{OperationCreatedEventData, OperationEvent};

#[async_trait]
pub trait OperationRepository {
    async fn persist_operation_created_event(&self, event_data: OperationCreatedEventData) -> Result<(), Error>;
}