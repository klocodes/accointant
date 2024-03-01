use serde::{Deserialize, Serialize};
use crate::features::operations::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount(f64);

impl Amount {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value <= 0.0 {
            return Err(
                DomainError::InvalidAmount("Amount must be greater than zero".to_string())
            );
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let amount = Amount::new(100.0).unwrap();
        assert_eq!(amount.value(), 100.0);
    }

    #[test]
    fn test_new_zero() {
        let amount = Amount::new(0.0);
        assert!(amount.is_err());
    }

    #[test]
    fn test_new_negative() {
        let amount = Amount::new(-100.0);
        assert!(amount.is_err());
    }
}
