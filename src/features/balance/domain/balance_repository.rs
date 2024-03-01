use async_trait::async_trait;
use crate::features::balance::domain::events::balance_changed::BalanceChanged;
use crate::features::balance::error::BalanceError;
use crate::features::balance::infrastructure::error::InfrastructureError;

#[async_trait]
pub trait BalanceRepository {
    async fn persist_balance_changed_event(&self, balance_changed: &BalanceChanged) -> Result<(), BalanceError>;
}

pub struct MockBalanceRepository {
    has_error: bool,
}

impl MockBalanceRepository {
    pub fn new(has_error: bool) -> Self {
        Self { has_error }
    }
}

#[async_trait]
impl BalanceRepository for MockBalanceRepository {
    async fn persist_balance_changed_event(&self, _balance_changed: &BalanceChanged) -> Result<(), BalanceError> {
        if self.has_error {
            Err(
                BalanceError::Infrastructure(
                    InfrastructureError::Repository("Failed to persist balance event".to_string())
                )
            )
        } else {
            Ok(())
        }
    }
}