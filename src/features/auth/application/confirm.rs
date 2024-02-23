use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::confirm_registration::RequestData;
use crate::services::tokenizer::Tokenizer;

pub struct ConfirmRegistration;

impl ConfirmRegistration {
    pub async fn exec(rep: impl UserRepository, tokenizer: impl Tokenizer, data: RequestData) -> Result<(), Error> {
        let id = data.id()?;

        let mut user: User = rep.find_by_id(id)
            .await?
            .ok_or(
                Error::Client(BadRequest {
                    message: Some( "User not found by this id".into())
                })
            )?;
        let token = data.token();

        let is_valid = tokenizer.validate(token);

        if is_valid.is_err() {
            return Err(is_valid.unwrap_err());
        }

        user.confirm(token.to_string())?;

        rep.confirm_email(user).await?;

        Ok(())
    }
}