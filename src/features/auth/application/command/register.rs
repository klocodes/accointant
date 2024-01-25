use std::collections::HashMap;
use crate::bootstrap::app_context::TransactionManager;
use crate::db::db_transaction::DbTransaction;
use crate::errors::client::ClientErrors;
use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::registration::RequestData;
use crate::services::mailer::Mailer;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

pub struct RegisterCommand;

impl RegisterCommand {
    pub async fn exec<M>(
        mut transaction_manager: TransactionManager,
        rep: impl UserRepository,
        tokenizer: Tokenizer,
        mailer: M,
        templater: Templater<'_>,
        template_name: &str,
        request_data: RequestData,
    ) -> Result<(), Error>
        where
            M: Mailer
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

        let email = request_data.email().to_string();
        let password = request_data.password().to_string();
        let confirmation_token = tokenizer.generate()?;

        let user = User::register(email.clone(), password.clone(), password, confirmation_token.clone())?;

        let _ = rep.create(&mut transaction_manager, &user).await?;

        let mut body_data = HashMap::new();
        let url = format!(
            "http://localhost:8080/auth/confirm?email={}&token={}", email, confirmation_token
        );
        body_data.insert("url", url);

        let body = templater.render(template_name, body_data)?;

        let res = mailer.send(user.email().value().to_string(), "Confirmation email".to_string(), body).await;

        if let Err(e) = res {
            transaction_manager.rollback().await?;

            return Err(e);
        }

        transaction_manager.commit().await?;

        Ok(())
    }
}