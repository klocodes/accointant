use async_trait::async_trait;
use crate::events::event::Event;
use crate::features::balance::application::commands::change::command::ChangeCommand;
use crate::features::balance::domain::balance::Balance;
use crate::features::balance::domain::balance_repository::BalanceRepository;
use crate::features::balance::domain::events::balance_event::BalanceEvent;
use crate::features::balance::error::BalanceError;
use crate::support::command_bus::CommandHandler;
use crate::support::error::FeatureError;

pub struct ChangeCommandHandler<R: BalanceRepository + Send + Sync> {
    rep: R,
}

impl<R> ChangeCommandHandler<R>
    where
        R: BalanceRepository + Send + Sync,
{
    pub fn new(rep: R) -> Self {
        Self { rep }
    }
}

#[async_trait]
impl<R> CommandHandler<ChangeCommand> for ChangeCommandHandler<R>
    where
        R: BalanceRepository + Send + Sync,
{
    async fn handle(&mut self, command: ChangeCommand) -> Result<Vec<Event>, FeatureError> {
        let balance_event = Balance::handle_change(command)
            .map_err(|e|
                FeatureError::Balance(
                    BalanceError::Domain(e)
                )
            )?;
        let balance_changed = match balance_event.clone() {
            BalanceEvent::BalanceChanged(balance_changed) => balance_changed,
        };

        self.rep.persist_balance_changed_event(&balance_changed)
            .await
            .map_err(|e| FeatureError::Balance(e))?;

        Ok(vec![Event::BalanceEvent(balance_event)])
    }
}

#[cfg(test)]
mod tests {
    use crate::features::balance::domain::balance_repository::MockBalanceRepository;
    use crate::support::id::Id;
    use super::*;

    #[tokio::test]
    async fn test_handle_successful() {
        let rep = MockBalanceRepository::new(false);

        let mut handler = ChangeCommandHandler::new(rep);
        let command = create_command_fixture();

        let result = handler.handle(command).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_error() {
        let rep = MockBalanceRepository::new(true);

        let mut handler = ChangeCommandHandler::new(rep);
        let command = create_command_fixture();

        let result = handler.handle(command).await;
        assert!(result.is_err());
    }

    fn create_command_fixture() -> ChangeCommand {
        ChangeCommand::new(
            Id::generate(),
            100.0,
            "USD".to_string(),
            100.0,
            1.0,
        )
    }
}