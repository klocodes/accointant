use crate::features::balance::application::commands::change::command::ChangeCommand;
use crate::features::balance::domain::currency::Currency;
use crate::features::balance::domain::error::DomainError;
use crate::features::balance::domain::events::balance_changed::BalanceChanged;
use crate::features::balance::domain::events::balance_event::BalanceEvent;
use crate::features::balance::domain::rate::Rate;
use crate::support::id::Id;

pub struct Balance {
    user_id: Id,
    amount: f64,
    currency: Currency,
    currency_amount: f64,
    rate: Rate,
}

impl Balance {
    pub fn handle_change(command: ChangeCommand) -> Result<BalanceEvent, DomainError> {
        let balance = Balance {
            user_id: Id::new(command.user_id()),
            amount: command.amount(),
            currency: Currency::new(command.currency())?,
            currency_amount: command.currency_amount(),
            rate: Rate::new(command.rate())?,
        };

        let event = BalanceEvent::BalanceChanged(
            BalanceChanged::new(
                Id::new(Id::generate()),
                balance.user_id,
                balance.amount,
                balance.currency,
                balance.currency_amount,
                balance.rate,
            )
        );

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_change_successful() {
        let command = ChangeCommand::new(
            Id::generate(),
            1.0,
            "USD".to_string(),
            1.0,
            1.0,
        );
        let res = Balance::handle_change(command.clone());

        assert!(res.is_ok());

        let event = res.unwrap();

        let balance_changed = match event {
            BalanceEvent::BalanceChanged(balance_changed) => balance_changed,
        };

        assert_eq!(balance_changed.payload().amount(), command.amount());
        assert_eq!(balance_changed.payload().currency().to_str(), command.currency());
        assert_eq!(balance_changed.payload().currency_amount(), command.currency_amount());
        assert_eq!(balance_changed.payload().rate().rate(), command.rate());
    }

    #[test]
    fn test_handle_change_with_error() {
        let command = ChangeCommand::new(
            Id::generate(),
            0.0,
            "test".to_string(),
            0.0,
            -1.0,
        );
        let res = Balance::handle_change(command);

        assert!(res.is_err());
    }
}