use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OperationApplyingData {
    account_id: Uuid,
    operation_amount: f64,
    operation_kind: String,
}

impl OperationApplyingData {
    pub fn new(
        account_id: Uuid,
        operation_amount: f64,
        operation_kind: String,
    ) -> Self {
        Self {
            account_id,
            operation_amount,
            operation_kind,
        }
    }
    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn operation_amount(&self) -> f64 {
        self.operation_amount
    }

    pub fn operation_kind(&self) -> &str {
        &self.operation_kind
    }
}