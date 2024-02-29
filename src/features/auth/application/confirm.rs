use uuid::Uuid;
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::services::tokenizer::Tokenizer;

pub struct ConfirmRegistration {
    user_id: Uuid,
    token: String,
}

impl ConfirmRegistration {
    pub fn new(user_id: Uuid, token: String) -> Self {
        Self {
            user_id,
            token,
        }
    }

    pub async fn exec(
        &self,
        rep: impl UserRepository,
        tokenizer_adapter: TokenizerAdapter<impl Tokenizer>
    ) -> Result<(), AuthError> {
        let mut user: User = rep.find_by_id(self.user_id)
            .await?
            .ok_or(
                AuthError::Domain(
                    DomainError::UserNotFound
                )
            )?;

        tokenizer_adapter.validate(self.token.as_str())?;


        user.confirm(self.token.clone())
            .map_err(|e| AuthError::Domain(e))?;

        rep.confirm_email(user).await?;

        Ok(())
    }
}