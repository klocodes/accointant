use async_trait::async_trait;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::operations::domain::events::operation_created::OperationCreated;

#[async_trait]
pub trait OperationRepository {
    async fn persist_operation_created_event(&self, event_data: OperationCreated) -> Result<(), Error>;
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
    async fn persist_operation_created_event(&self, _event_data: OperationCreated) -> Result<(), Error> {
        if self.has_error {
            return Err(Error::Server(
                InternalServerError {
                    context: Some("Error".into())
                }
            ));
        }

        Ok(())
    }
}