use async_trait::async_trait;
use crate::db::transaction::container::TransactionContainer;
use crate::errors::Error;
use crate::features::operations::domain::events::operation_created::OperationCreated;

#[async_trait]
pub trait OperationRepository {
    async fn persist_operation_created_event(&self, transaction_container: &mut TransactionContainer, event_data: OperationCreated) -> Result<(), Error>;
}