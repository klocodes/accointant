use uuid::Uuid;
use crate::support::command_bus::Command;

const NAME: &str = "change_command";

#[derive(Clone, Debug)]
pub struct ChangeCommand {
    user_id: Uuid,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
}

impl ChangeCommand {
    pub fn new(user_id: Uuid, amount: f64, currency: String, currency_amount: f64, rate: f64) -> Self {
        Self {
            user_id,
            amount,
            currency,
            currency_amount,
            rate,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn currency_amount(&self) -> f64 {
        self.currency_amount
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

impl Command for ChangeCommand {
    fn name() -> &'static str {
        NAME
    }
}

