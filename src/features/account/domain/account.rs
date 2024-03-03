use crate::features::account::domain::currency::Currency;
use crate::features::account::domain::dto::creation_command::CreationCommand;
use crate::features::account::domain::error::DomainError;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::domain::rate::Rate;
use crate::features::account::domain::source::Source;
use crate::support::id::Id;

pub struct Account {
    id: Id,
    user_id: Id,
    name: String,
    amount: f64,
    currency: Currency,
    currency_amount: f64,
    rate: Rate,
    icon: String,
    source: Option<Source>,
}

impl Account {
    pub fn handle_creation(command: CreationCommand) -> Result<AccountEvent, DomainError> {
        let amount = command.amount().clone();
        let currency_amount = command.currency_amount().clone();
        let rate = Rate::new(command.rate().clone())?;

        if amount != currency_amount / rate.value() {
            return Err(
                DomainError::InvalidAmount(
                    format!("Amount {} is not equal to currency amount {} by rate {}", amount, currency_amount, rate.value())
                )
            );
        }

        let source = match command.source() {
            Some(source) => Some(Source::new(source.clone())?),
            None => None,
        };

        let account = Account {
            id: Id::new(Id::generate()),
            user_id: Id::new(command.user_id()),
            name: command.name().to_string(),
            amount: amount.clone(),
            currency: Currency::new(command.currency())?,
            currency_amount: currency_amount.clone(),
            rate,
            icon: command.icon().to_string(),
            source
        };

        let account_created = AccountCreated::new(
            Id::new(Id::generate()),
            account.id,
            account.user_id,
            account.name,
            account.amount,
            account.currency,
            account.currency_amount,
            account.rate,
            account.icon,
            account.source,
        );

        Ok(AccountEvent::AccountCreated(account_created))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation_successful() {
        let command = command();
        let account_event = Account::handle_creation(command.clone()).unwrap();

        match account_event {
            AccountEvent::AccountCreated(account_created) => {
                assert_eq!(account_created.payload().account_name(), command.name());
                assert_eq!(account_created.payload().amount(), command.amount());
                assert_eq!(account_created.payload().currency_amount(), command.currency_amount());
                assert_eq!(account_created.payload().icon(), command.icon());
                assert_eq!(account_created.payload().source().clone().unwrap().to_str(), command.source().clone().unwrap());
            }
        }
    }

    #[test]
    fn test_handle_creation_with_invalid_currency() {
        let command = command();
        let command = CreationCommand::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            "invalid_currency".to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            command.source().clone(),
        );

        let account_event = Account::handle_creation(command.clone());

        match account_event {
            Err(DomainError::UnknownCurrency(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_handle_creation_with_invalid_rate() {
        let command = command();
        let command = CreationCommand::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            command.currency().to_string(),
            command.currency_amount().clone(),
            0.0,
            command.icon().to_string(),
            command.source().clone()
        );

        let account_event = Account::handle_creation(command.clone());

        match account_event {
            Err(DomainError::RateMustBePositive) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_handle_creation_with_invalid_source() {
        let command = command();
        let command = CreationCommand::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            command.currency().to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            Some("invalid_source".to_string())
        );

        let account_event = Account::handle_creation(command.clone());

        match account_event {
            Err(DomainError::UnknownSource(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_handle_creation_with_invalid_amount() {
        let command = command();
        let command = CreationCommand::new(
            command.user_id(),
            command.name().to_string(),
            10.0,
            command.currency().to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            command.source().clone(),
        );

        let account_event = Account::handle_creation(command.clone());

        match account_event {
            Err(DomainError::InvalidAmount(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    fn command() -> CreationCommand {
        CreationCommand::new(
            Id::generate(),
            "Kaspi Gold".to_string(),
            100.0,
            "USD".to_string(),
            100.0,
            1.0,
            "icon".to_string(),
            Some("Kaspi".to_string()),
        )
    }
}