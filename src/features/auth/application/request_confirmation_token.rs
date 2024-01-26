use std::collections::HashMap;
use uuid::Uuid;
use crate::db::transaction::container::TransactionContainer;
use crate::db::transaction::manager::TransactionManager as TransactionManagerTrait;

use crate::errors::client::ClientErrors::{BadRequest};
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::services::mailer::Mailer;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

pub struct RequestConfirmationToken;

impl RequestConfirmationToken {
    pub async fn exec<M>(
        mut transaction_container: TransactionContainer<'_>,
        rep: impl UserRepository,
        tokenizer: impl Tokenizer,
        mailer: M,
        templater: impl Templater,
        template_name: &str,
        user_id: &str,
    ) -> Result<(), Error>
        where
            M: Mailer
    {
        let user_id = Uuid::parse_str(user_id)
            .map_err(|e| Error::Client(BadRequest {
                message: Some(format!("Failed to parse user id: {}", e.to_string()).into())
            }))?;

        let mut user: User = rep.find_by_id(user_id)
            .await?
            .ok_or(
                Error::Client(BadRequest {
                    message: Some( "User not found by this email".into())
                })
            )?;

        let token = tokenizer.generate()?;
        user.request_confirmation(token.clone()).await?;

        rep.update_confirmation_token(&mut transaction_container, user.clone()).await?;

        let mut body_data = HashMap::new();
        let url = format!(
            "http://localhost:8080/auth/confirm/{}?token={}", user_id, token
        );
        body_data.insert("url", url);

        let body = templater.render(template_name, body_data)?;
        let email = user.email().value().to_string();
        let subject = "Confirmation email".to_string();

        let res = mailer.send(email, subject, body).await;

        if let Err(e) = res {
            transaction_container.take_manager().rollback().await?;

            return Err(e);
        }

        transaction_container.take_manager().commit().await?;

        Ok(())
    }
}