use uuid::Uuid;
use crate::features::account::domain::dto::creation_command::CreationCommand;
use crate::support::command_bus::Command;

const NAME: &str = "create_account_command";

pub struct CreateAccountCommand {
    user_id: Uuid,
    name: String,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
    icon: String,
    source: Option<String>,
}

impl CreateAccountCommand {
    pub fn new(
        user_id: Uuid,
        name: String,
        amount: f64,
        currency: String,
        currency_amount: f64,
        rate: f64,
        icon: String,
        source: Option<String>,
    ) -> Self {
        Self {
            user_id,
            name,
            amount,
            currency,
            currency_amount,
            rate,
            icon,
            source,
        }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
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

    pub fn icon(&self) -> &str {
        &self.icon
    }

    pub fn source(&self) -> &Option<String> {
        &self.source
    }
}

impl Command for CreateAccountCommand {
    fn name() -> &'static str {
        NAME
    }
}

impl From<CreateAccountCommand> for CreationCommand {
    fn from(command: CreateAccountCommand) -> Self {
        CreationCommand::new(
            command.user_id,
            command.name,
            command.amount,
            command.currency,
            command.currency_amount,
            command.rate,
            command.icon,
            command.source,
        )
    }
}