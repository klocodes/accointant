use rand::distributions::{Alphanumeric, Uniform};
use rand::Rng;
use regex::Regex;

use crate::errors::client::ClientErrors::DomainError;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

const LENGTH: usize = 32;
const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?";


pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self) -> Result<String, Error> {
        let rng = rand::thread_rng();
        let symbols = Uniform::new_inclusive(0, SYMBOLS.chars().count() - 1);
        let value: String = rng
            .sample_iter(symbols)
            .take(LENGTH)
            .map(|i| SYMBOLS.chars().nth(i).unwrap())
            .collect();

        Ok(value)
    }

    pub fn validate(&self, value: &str) -> Result<(), Error> {
        let has_uppercase = Regex::new(r"[A-Z]")
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to validate confirmation token: {}", e.to_string()).into())
                    }
                )
            )?;
        let has_lowercase = Regex::new(r"[a-z]")
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to validate confirmation token: {}", e.to_string()).into())
                    }
                )
            )?;
        let has_number = Regex::new(r"\d")
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to validate confirmation token: {}", e.to_string()).into())
                    }
                )
            )?;
        let has_special = Regex::new(r"[^A-Za-z0-9]")
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(format!("Failed to validate confirmation token: {}", e.to_string()).into())
                    }
                )
            )?;

        if !has_uppercase.is_match(value) ||
            !has_lowercase.is_match(value) ||
            !has_number.is_match(value) ||
            !has_special.is_match(value) ||
            value.len() < LENGTH
        {
            return Err(
                Error::Client(
                    DomainError {
                        message: "Token is non valid".into()
                    }
                )
            );
        }

        Ok(())
    }
}