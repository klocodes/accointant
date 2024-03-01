use mockall::automock;
use rand::distributions::Uniform;
use rand::Rng;
use regex::Regex;
use crate::services::error::ServiceError;

const LENGTH: usize = 32;
const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";


#[automock]
pub trait Tokenizer {
    fn generate(&self) -> Result<String, ServiceError>;
    fn validate(&self, value: &str) -> Result<(), ServiceError>;
}

pub struct SymbolsTokenizer;

impl SymbolsTokenizer {
    pub fn new() -> Self {
        Self
    }
}

impl Tokenizer for SymbolsTokenizer {
    fn generate(&self) -> Result<String, ServiceError> {
        let rng = rand::thread_rng();
        let symbols = Uniform::new_inclusive(0, SYMBOLS.chars().count() - 1);
        let value: String = rng
            .sample_iter(symbols)
            .take(LENGTH)
            .map(|i| SYMBOLS.chars().nth(i).unwrap())
            .collect();

        Ok(value)
    }

    fn validate(&self, value: &str) -> Result<(), ServiceError> {
        let has_uppercase = Regex::new(r"[A-Z]")
            .map_err(|e|
                ServiceError::Tokenizer(e.to_string())
            )?;
        let has_lowercase = Regex::new(r"[a-z]")
            .map_err(|e|
                ServiceError::Tokenizer(e.to_string())
            )?;
        let has_number = Regex::new(r"\d")
            .map_err(|e|
                ServiceError::Tokenizer(e.to_string())
            )?;

        if !has_uppercase.is_match(value) ||
            !has_lowercase.is_match(value) ||
            !has_number.is_match(value) ||
            value.len() < LENGTH
        {
            return Err(
                ServiceError::Tokenizer("Invalid token".to_string())
            );
        }

        Ok(())
    }
}