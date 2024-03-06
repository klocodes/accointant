use async_trait::async_trait;
use crate::events::event::Event;
use crate::features::account::application::commands::apply_operation::command::ApplyOperationCommand;
use crate::features::account::domain::account::Account;
use crate::features::account::domain::account_repository::AccountRepository;
use crate::features::account::domain::dto::operation_applying_data::OperationApplyingData;
use crate::features::account::domain::error::DomainError;
use crate::features::account::error::AccountError;
use crate::support::command_bus::CommandHandler;
use crate::support::error::FeatureError;

pub struct ApplyOperationCommandHandler<R>
    where
        R: AccountRepository + Send + Sync,
{
    rep: R,
}

impl<R> ApplyOperationCommandHandler<R>
    where
        R: AccountRepository + Send + Sync,
{
    pub fn new(rep: R) -> Self {
        Self { rep }
    }
}

#[async_trait]
impl<R> CommandHandler<ApplyOperationCommand> for ApplyOperationCommandHandler<R>
    where
        R: AccountRepository + Send + Sync,
{
    async fn handle(&mut self, command: ApplyOperationCommand) -> Result<Vec<Event>, FeatureError> {
        let account_events = self.rep.find_events_by_id(command.account_id())
            .await
            .map_err(|e|
                FeatureError::Account(e)
            )?;

        if account_events.is_empty() {
            return Err(
                FeatureError::Account(
                    AccountError::Domain(
                        DomainError::AccountNotFound
                    )
                )
            );
        }

        let account = Account::recover_state(command.account_id(), account_events)
            .map_err(|e|
                FeatureError::Account(
                    AccountError::Domain(e)
                )
            )?;

        let data = OperationApplyingData::from(command);
        let event = account.apply_operation(data)
            .map_err(|e|
                FeatureError::Account(
                    AccountError::Domain(e)
                )
            )?;


        Ok(vec![Event::AccountEvent(event)])
    }
}