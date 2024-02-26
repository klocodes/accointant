use async_trait::async_trait;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::balance::domain::events::balance_changed::BalanceChanged;

#[async_trait]
pub trait BalanceRepository {
    async fn persist_balance_changed_event(&self, balance_changed: &BalanceChanged) -> Result<(), Error>;
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
    async fn persist_balance_changed_event(&self, _balance_changed: &BalanceChanged) -> Result<(), Error> {
        if self.has_error {
            Err(Error::Server(
                InternalServerError {
                    context: Some("Failed to persist balance event".into())
                }
            ))
        } else {
            Ok(())
        }
    }
}