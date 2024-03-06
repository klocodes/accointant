use async_trait::async_trait;
use uuid::Uuid;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::domain::events::account_operation_applied::AccountOperationApplied;
use crate::features::account::error::AccountError;
use crate::features::account::infrastructure::error::InfrastructureError;

#[async_trait]
pub trait AccountRepository {
    async fn find_events_by_id(&self, account_id: Uuid) -> Result<Vec<AccountEvent>, AccountError>;

    async fn persist_account_created_event(&self, event: AccountCreated) -> Result<(), AccountError>;

    async fn persist_account_operation_applied_event(&self, event: AccountOperationApplied) -> Result<(), AccountError>;
}


pub struct MockAccountRepository {
    has_error: bool,
}

impl MockAccountRepository {
    pub fn new(has_error: bool) -> Self {
        Self {
            has_error,
        }
    }
}

#[async_trait]
impl AccountRepository for MockAccountRepository {
    async fn find_events_by_id(&self, _account_id: Uuid) -> Result<Vec<AccountEvent>, AccountError> {
        if self.has_error {
            Err(AccountError::Infrastructure(
                InfrastructureError::Repository("Mock repository error".to_string())
            ))
        } else {
            Ok(vec![])
        }
    }

    async fn persist_account_created_event(&self, _event: AccountCreated) -> Result<(), AccountError> {
        if self.has_error {
            Err(AccountError::Infrastructure(
                InfrastructureError::Repository("Mock repository error".to_string())
            ))
        } else {
            Ok(())
        }
    }

    async fn persist_account_operation_applied_event(&self, _event: AccountOperationApplied) -> Result<(), AccountError> {
        if self.has_error {
            Err(AccountError::Infrastructure(
                InfrastructureError::Repository("Mock repository error".to_string())
            ))
        } else {
            Ok(())
        }
    }
}