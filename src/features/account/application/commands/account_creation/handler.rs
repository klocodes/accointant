use async_trait::async_trait;
use crate::events::event::Event;
use crate::features::account::application::commands::account_creation::command::CreateAccountCommand;
use crate::features::account::domain::account::Account;
use crate::features::account::domain::account_repository::AccountRepository;
use crate::features::account::domain::dto::creation_data::CreationData;
use crate::features::account::domain::error::DomainError;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::error::AccountError;
use crate::support::command_bus::CommandHandler;
use crate::support::error::FeatureError;

pub struct CreateAccountCommandHandler<R>
    where R: AccountRepository + Send + Sync
{
    account_repository: R,
}

impl<R> CreateAccountCommandHandler<R>
    where R: AccountRepository + Send + Sync
{
    pub fn new(account_repository: R) -> Self {
        Self {
            account_repository,
        }
    }
}

#[async_trait]
impl<R> CommandHandler<CreateAccountCommand> for CreateAccountCommandHandler<R>
    where R: AccountRepository + Send + Sync
{
    async fn handle(&mut self, command: CreateAccountCommand) -> Result<Vec<Event>, FeatureError> {
        let account_event = Account::create(CreationData::from(command))
            .map_err(|e| FeatureError::Account(
                AccountError::Domain(e)
            ))?;

        let account_created = match account_event.clone() {
            AccountEvent::AccountCreated(account_created) => Ok(account_created),
            _ => {
                Err(
                    FeatureError::Account(
                        AccountError::Domain(
                            DomainError::InvalidEvent("AccountCreatedEvent not found".to_string())
                        )
                    )
                )
            }
        }?;

        self.account_repository.persist_account_created_event(account_created)
            .await
            .map_err(|e| FeatureError::Account(e))?;

        Ok(vec![Event::AccountEvent(account_event)])
    }
}

#[cfg(test)]
mod tests {
    use crate::features::account::domain::account_repository::MockAccountRepository;
    use crate::support::id::Id;
    use super::*;

    #[tokio::test]
    async fn test_account_creation_handler_success() {
        let account_repository = MockAccountRepository::new(false);

        let mut handler = CreateAccountCommandHandler::new(account_repository);
        let command = command();

        let result = handler.handle(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_account_creation_handler_repository_error() {
        let account_repository = MockAccountRepository::new(true);

        let mut handler = CreateAccountCommandHandler::new(account_repository);
        let command = command();

        let result = handler.handle(command).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        let error_is_correct = match error {
            FeatureError::Account(e) => match e {
                AccountError::Infrastructure(e) => true,
                _ => false,
            },
            _ => false,
        };

        assert!(error_is_correct);
    }

    #[tokio::test]
    async fn test_account_creation_handler_with_domain_error() {
        let account_repository = MockAccountRepository::new(false);

        let mut handler = CreateAccountCommandHandler::new(account_repository);
        let correct_command = command();
        let command = CreateAccountCommand::new(
            correct_command.user_id().clone(),
            correct_command.name().to_string(),
            correct_command.amount(),
            correct_command.currency().to_string(),
            correct_command.currency_amount(),
            0.0,
            correct_command.icon().to_string(),
            correct_command.source().clone(),
        );

        let result = handler.handle(command).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        let error_is_correct = match error {
            FeatureError::Account(e) => match e {
                AccountError::Domain(e) => true,
                _ => false,
            },
            _ => false,
        };

        assert!(error_is_correct);
    }

    fn command() -> CreateAccountCommand {
        CreateAccountCommand::new(
            Id::generate(),
            "Kaspi Gold".to_string(),
            1000.0,
            "KZT".to_string(),
            500000.0,
            500.0,
            "icon".to_string(),
            Some("Kaspi".to_string())
        )
    }
}