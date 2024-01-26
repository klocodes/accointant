use std::collections::HashMap;
use crate::db::transaction::container::TransactionContainer;
use crate::db::transaction::manager::TransactionManager as TransactionManagerTrait;
use crate::errors::client::ClientErrors;
use crate::errors::Error;
use crate::features::auth::application::dto::user_data::UserData;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::registration::RequestData;
use crate::services::hasher::Hasher;
use crate::services::mailer::Mailer;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

pub struct RegisterUser;

impl RegisterUser {
    pub async fn exec(
        mut transaction_container: TransactionContainer<'_>,
        rep: impl UserRepository,
        hasher: impl Hasher,
        tokenizer: impl Tokenizer,
        mailer: impl Mailer,
        templater: impl Templater,
        template_name: &str,
        request_data: RequestData,
    ) -> Result<(), Error>
    {
        let email_exists: bool = rep.email_exists(request_data.email()).await?;

        if email_exists {
            return Err(Error::Client(ClientErrors::BadRequest {
                message: Some(
                    format!(
                        "User with email {} already exists", request_data.email()
                    ).into()
                )
            }));
        }

        let user_data = UserData::new(
            request_data.email().to_string(),
            request_data.password().to_string(),
            request_data.password_confirmation().to_string(),
            hasher.hash(request_data.password().to_string())?,
            tokenizer.generate()?,
        );
        let user = User::register(user_data.clone())?;

        let _ = rep.create(&mut transaction_container, &user).await?;

        let mut body_data = HashMap::new();
        let url = format!(
            "http://localhost:8080/auth/confirm?email={}&token={}", user_data.email(), user_data.confirmation_token()
        );
        body_data.insert("url", url);

        let body = templater.render(template_name, body_data)?;

        let res = mailer.send(user.email().value().to_string(), "Confirmation email".to_string(), body).await;

        if let Err(e) = res {
            transaction_container.take_manager().rollback().await?;

            return Err(e);
        }

        transaction_container.take_manager().commit().await?;

        Ok(())
    }
}
