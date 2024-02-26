use serde::{Deserialize, Serialize};
use crate::errors::client::ClientErrors::DomainError;
use crate::errors::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rate {
    rate: f64,
}

impl Rate {
    pub fn new(rate: f64) -> Result<Self, Error> {
        if rate <= 0.0 {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Rate cannot be negative".into(),
                    }
                )
            )
        }

        Ok(
            Self { rate }
        )
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rate() {
        let rate = Rate::new(0.1);

        assert!(rate.is_ok());
    }

    #[test]
    fn test_new_rate_negative() {
        let rate = Rate::new(-1.0);
        assert!(rate.is_err());
    }

    #[test]
    fn test_new_rate_zero() {
        let rate = Rate::new(0.0);
        assert!(rate.is_err());
    }
}