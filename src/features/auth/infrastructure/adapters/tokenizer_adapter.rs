use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::tokenizer::Tokenizer;

pub struct TokenizerAdapter<T: Tokenizer> {
    tokenizer: T,
}

impl<T: Tokenizer> TokenizerAdapter<T> {
    pub fn new(tokenizer: T) -> Self {
        TokenizerAdapter { tokenizer }
    }

    pub fn generate(&self) -> Result<String, AuthError> {
        self.tokenizer.generate()
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Tokenizer(e.to_string())
                )
            )
    }

    pub fn validate(&self, token: &str) -> Result<(), AuthError> {
        self.tokenizer.validate(token)
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Tokenizer(e.to_string())
                )
            )
    }
}