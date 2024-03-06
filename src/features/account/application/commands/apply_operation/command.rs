use uuid::Uuid;
use crate::features::account::domain::dto::operation_applying_data::OperationApplyingData;
use crate::support::command_bus::Command;

const NAME: &str = "apply_operation_command";

pub struct ApplyOperationCommand {
    account_id: Uuid,
    operation_id: Uuid,
    amount: f64,
    operation_kind: String,
}

impl ApplyOperationCommand {
    pub fn new(
        account_id: Uuid,
        operation_id: Uuid,
        amount: f64,
        operation_kind: String,
    ) -> Self {
        Self {
            account_id,
            operation_id,
            amount,
            operation_kind,
        }
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn operation_id(&self) -> Uuid {
        self.operation_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn operation_kind(&self) -> &str {
        &self.operation_kind
    }
}

impl Command for ApplyOperationCommand {
    fn name() -> &'static str {
        NAME
    }
}

impl From<ApplyOperationCommand> for OperationApplyingData {
    fn from(command: ApplyOperationCommand) -> Self {
        OperationApplyingData::new(
            command.account_id(),
            command.amount(),
            command.operation_kind().to_string(),
        )
    }
}