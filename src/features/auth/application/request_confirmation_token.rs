use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::db::manager::DbManager;
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::adapters::mailer_adapter::MailerAdapter;
use crate::features::auth::infrastructure::adapters::templater_adapter::TemplaterAdapter;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::mailer::Mailer;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;
use crate::support::error::FeatureError;

pub struct RequestConfirmationToken;

impl RequestConfirmationToken {
    pub async fn exec(
        db_manager: Arc<Mutex<DbManager>>,
        rep: impl UserRepository,
        tokenizer: TokenizerAdapter<impl Tokenizer>,
        mailer: MailerAdapter<impl Mailer>,
        templater: TemplaterAdapter<impl Templater>,
        template_name: &str,
        user_id: Uuid,
    ) -> Result<(), FeatureError>
    {
        let mut user: User = rep.find_by_id(user_id)
            .await
            .map_err(|e| FeatureError::Auth(e))?
            .ok_or(
                FeatureError::Auth(
                    AuthError::Domain(
                        DomainError::UserNotFound
                    )
                )
            )?;

        let token = tokenizer.generate()
            .map_err(|e| FeatureError::Auth(e))?;
        user.request_confirmation(token.clone())
            .map_err(|e|
                FeatureError::Auth(
                    AuthError::Domain(e)
                )
            )?;

        rep.update_confirmation_token(user.clone())
            .await
            .map_err(|e| FeatureError::Auth(e))?;

        let mut body_data = HashMap::new();
        let url = format!(
            "http://localhost:8080/auth/confirm/{}?token={}", user_id, token
        );
        body_data.insert("url", url);

        let body = templater.render(template_name, body_data)
            .map_err(|e| FeatureError::Auth(e))?;
        let email = user.email().value().to_string();
        let subject = "Confirmation email".to_string();

        let res = mailer.send(email, subject, body).await;

        if let Err(e) = res {
            let mut guard = db_manager.lock().await;

            guard.rollback()
                .await
                .map_err(|e|
                    FeatureError::Auth(
                        AuthError::Infrastructure(
                            InfrastructureError::Transaction(e.to_string())
                        )
                    )
                )?;

            return Err(FeatureError::Auth(e));
        }

        let mut guard = db_manager.lock().await;

        guard.commit()
            .await
            .map_err(|e|
                FeatureError::Auth(
                    AuthError::Infrastructure(
                        InfrastructureError::Transaction(e.to_string())
                    )
                )
            )?;

        Ok(())
    }
}