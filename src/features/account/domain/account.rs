use uuid::Uuid;
use crate::features::account::domain::currency::Currency;
use crate::features::account::domain::dto::creation_data::CreationData;
use crate::features::account::domain::dto::operation_applying_data::OperationApplyingData;
use crate::features::account::domain::error::DomainError;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::domain::events::account_operation_applied::AccountOperationApplied;
use crate::features::account::domain::operation_amount::OperationAmount;
use crate::features::account::domain::operation_kind::OperationKind;
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
    pub fn create(data: CreationData) -> Result<AccountEvent, DomainError> {
        let amount = data.amount().clone();
        let currency_amount = data.currency_amount().clone();
        let rate = Rate::new(data.rate().clone())?;

        if amount != currency_amount / rate.value() {
            return Err(
                DomainError::InvalidAmount(
                    format!("Amount {} is not equal to currency amount {} by rate {}", amount, currency_amount, rate.value())
                )
            );
        }

        let source = match data.source() {
            Some(source) => Some(Source::new(source.clone())?),
            None => None,
        };

        let account = Account {
            id: Id::new(Id::generate()),
            user_id: Id::new(data.user_id()),
            name: data.name().to_string(),
            amount: amount.clone(),
            currency: Currency::new(data.currency())?,
            currency_amount: currency_amount.clone(),
            rate,
            icon: data.icon().to_string(),
            source,
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

    pub fn apply_operation(&self, data: OperationApplyingData) -> Result<AccountEvent, DomainError> {
        let current_amount = self.amount.clone();
        let current_rate = self.rate.clone();

        let operation_amount = OperationAmount::new(data.operation_amount().clone())?;
        let operation_kind = OperationKind::new(data.operation_kind())?;

        let new_amount = match operation_kind {
            OperationKind::Income => current_amount + operation_amount.value(),
            OperationKind::Expense => current_amount - operation_amount.value(),
        };

        let new_currency_amount = match operation_kind {
            OperationKind::Income => current_amount + operation_amount.value() * current_rate.value(),
            OperationKind::Expense => current_amount - operation_amount.value() * current_rate.value(),
        };

        let account_operation_applied = AccountOperationApplied::new(
            Id::new(Id::generate()),
            self.id.clone(),
            new_amount,
            new_currency_amount,
            current_rate,
            operation_kind,
        );

        Ok(AccountEvent::AccountOperationApplied(account_operation_applied))
    }

    pub fn recover_state(_id: Uuid, events: Vec<AccountEvent>) -> Result<Account, DomainError> {
        let mut account: Option<Account> = None;

        for event in events {
            account = match event {
                AccountEvent::AccountCreated(account_created) => {
                    if (account.is_some()) {
                        return Err(DomainError::AccountNotFound);
                    }

                    Some(
                        Account {
                            id: account_created.payload().account_id().clone(),
                            user_id: account_created.payload().user_id().clone(),
                            name: account_created.payload().account_name().to_string(),
                            amount: *account_created.payload().amount(),
                            currency: account_created.payload().currency().clone(),
                            currency_amount: *account_created.payload().currency_amount(),
                            rate: *account_created.payload().rate(),
                            icon: account_created.payload().icon().to_string(),
                            source: account_created.payload().source().clone(),
                        }
                    )
                }
                AccountEvent::AccountOperationApplied(account_operation_applied) => {
                    let new_amount = account_operation_applied.payload().new_amount();
                    let new_currency_amount = account_operation_applied.payload().new_currency_amount();
                    let current_rate = account_operation_applied.payload().current_rate();

                    let mut account = account.ok_or(DomainError::AccountNotFound)?;

                    account.amount = new_amount;
                    account.currency_amount = new_currency_amount;
                    account.rate = current_rate;

                    Some(account)
                }
            }
        }

        Ok(
            account.ok_or(DomainError::AccountNotFound)?
        )
    }
}

#[cfg(test)]
mod creation_tests {
    use super::*;

    #[test]
    fn test_creation_with_success() {
        let command = creation_data_fixture();
        let account_event = Account::create(command.clone()).unwrap();

        match account_event {
            AccountEvent::AccountCreated(account_created) => {
                assert_eq!(account_created.payload().account_name(), command.name());
                assert_eq!(account_created.payload().amount(), command.amount());
                assert_eq!(account_created.payload().currency_amount(), command.currency_amount());
                assert_eq!(account_created.payload().icon(), command.icon());
                assert_eq!(account_created.payload().source().clone().unwrap().to_str(), command.source().clone().unwrap());
            }
            _ => {}
        }
    }

    #[test]
    fn test_creation_with_invalid_currency() {
        let command = creation_data_fixture();
        let command = CreationData::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            "invalid_currency".to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            command.source().clone(),
        );

        let account_event = Account::create(command.clone());

        match account_event {
            Err(DomainError::UnknownCurrency(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_creation_with_invalid_rate() {
        let command = creation_data_fixture();
        let command = CreationData::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            command.currency().to_string(),
            command.currency_amount().clone(),
            0.0,
            command.icon().to_string(),
            command.source().clone(),
        );

        let account_event = Account::create(command.clone());

        match account_event {
            Err(DomainError::RateMustBePositive) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_creation_with_invalid_source() {
        let command = creation_data_fixture();
        let command = CreationData::new(
            command.user_id(),
            command.name().to_string(),
            command.amount().clone(),
            command.currency().to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            Some("invalid_source".to_string()),
        );

        let account_event = Account::create(command.clone());

        match account_event {
            Err(DomainError::UnknownSource(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_creation_with_invalid_amount() {
        let command = creation_data_fixture();
        let command = CreationData::new(
            command.user_id(),
            command.name().to_string(),
            10.0,
            command.currency().to_string(),
            command.currency_amount().clone(),
            command.rate().clone(),
            command.icon().to_string(),
            command.source().clone(),
        );

        let account_event = Account::create(command.clone());

        match account_event {
            Err(DomainError::InvalidAmount(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    fn creation_data_fixture() -> CreationData {
        CreationData::new(
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

#[cfg(test)]
mod operation_applying_tests {
    use super::*;

    #[test]
    fn test_operation_applying_with_income() {
        let account = account_fixture();
        let data = operation_applying_data_fixture();
        let account_event_result = account.apply_operation(data.clone());

        assert!(account_event_result.is_ok());

        let account_event = account_event_result.unwrap();
        match account_event {
            AccountEvent::AccountOperationApplied(account_operation_applied) => {
                assert_eq!(account_operation_applied.payload().account_id(), account.id);
                assert_eq!(account_operation_applied.payload().new_amount(), account.amount + data.operation_amount());
                assert_eq!(account_operation_applied.payload().new_currency_amount(), account.amount + data.operation_amount() * account.rate.value());
                assert_eq!(account_operation_applied.payload().current_rate().value(), account.rate.value());
            }
            _ => {}
        }
    }

    #[test]
    fn test_operation_applying_with_expense() {
        let account = account_fixture();
        let data = OperationApplyingData::new(
            Id::generate(),
            10.0,
            "Expense".to_string(),
        );
        let account_event_result = account.apply_operation(data.clone());

        assert!(account_event_result.is_ok());

        let account_event = account_event_result.unwrap();
        match account_event {
            AccountEvent::AccountOperationApplied(account_operation_applied) => {
                assert_eq!(account_operation_applied.payload().account_id(), account.id);
                assert_eq!(account_operation_applied.payload().new_amount(), account.amount - data.operation_amount());
                assert_eq!(account_operation_applied.payload().new_currency_amount(), account.amount - data.operation_amount() * account.rate.value());
                assert_eq!(account_operation_applied.payload().current_rate().value(), account.rate.value());
            }
            _ => {}
        }
    }

    #[test]
    fn test_operation_applying_with_invalid_operation_kind() {
        let account = account_fixture();
        let data = OperationApplyingData::new(
            Id::generate(),
            10.0,
            "Invalid".to_string(),
        );
        let account_event_result = account.apply_operation(data.clone());

        assert!(account_event_result.is_err());

        match account_event_result {
            Err(DomainError::UnknownOperationKind(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_operation_applying_with_invalid_amount() {
        let account = account_fixture();
        let data = OperationApplyingData::new(
            Id::generate(),
            -10.0,
            "Income".to_string(),
        );
        let account_event_result = account.apply_operation(data.clone());

        assert!(account_event_result.is_err());

        match account_event_result {
            Err(DomainError::InvalidAmount(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    fn account_fixture() -> Account {
        Account {
            id: Id::new(Id::generate()),
            user_id: Id::new(Id::generate()),
            name: "Kaspi Gold".to_string(),
            amount: 100.0,
            currency: Currency::new("USD").unwrap(),
            currency_amount: 100.0,
            rate: Rate::new(1.0).unwrap(),
            icon: "icon".to_string(),
            source: Some(Source::new("Kaspi".to_string()).unwrap()),
        }
    }

    fn operation_applying_data_fixture() -> OperationApplyingData {
        OperationApplyingData::new(
            Id::generate(),
            10.0,
            "Income".to_string(),
        )
    }
}