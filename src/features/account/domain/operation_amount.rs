use crate::features::account::domain::error::DomainError;

pub struct OperationAmount(f64);

impl OperationAmount {
    pub fn new(amount: f64) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidAmount(
                format!("Operation amount {} must be more 0", amount)
            ));
        }

        Ok(Self(amount))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}