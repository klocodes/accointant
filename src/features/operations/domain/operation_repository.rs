use async_trait::async_trait;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::error::OperationError;
use crate::features::operations::infrastructure::error::InfrastructureError;

#[async_trait]
pub trait OperationRepository {
    async fn persist_operation_created_event(&self, event_data: OperationCreated) -> Result<(), OperationError>;
}

pub struct MockOperationRepository {
    has_error: bool,
}

impl MockOperationRepository {
    pub fn new(has_error: bool) -> Self {
        Self {
            has_error
        }
    }
}

#[async_trait]
impl OperationRepository for MockOperationRepository {
    async fn persist_operation_created_event(&self, _event_data: OperationCreated) -> Result<(), OperationError> {
        if self.has_error {
            return Err(OperationError::Infrastructure(
               InfrastructureError::Repository("Mock repository error".to_string())
            ));
        }

        Ok(())
    }
}