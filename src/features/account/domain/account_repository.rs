use async_trait::async_trait;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::error::AccountError;
use crate::features::account::infrastructure::error::InfrastructureError;

#[async_trait]
pub trait AccountRepository {
    async fn persist_account_created_event(&self, event: AccountCreated) -> Result<(), AccountError>;
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
    async fn persist_account_created_event(&self, _event: AccountCreated) -> Result<(), AccountError> {
        if self.has_error {
            Err(AccountError::Infrastructure(
                InfrastructureError::Repository("Mock repository error".to_string())
            ))
        } else {
            Ok(())
        }
    }

}