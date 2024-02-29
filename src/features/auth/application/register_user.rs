use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::db::manager::DbManager;
use crate::features::auth::application::dto::user_data::UserData;
use crate::features::auth::domain::error::DomainError;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::adapters::hasher_adapter::HasherAdapter;
use crate::features::auth::infrastructure::adapters::mailer_adapter::MailerAdapter;
use crate::features::auth::infrastructure::adapters::templater_adapter::TemplaterAdapter;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::http::handlers::auth::registration::RequestData;
use crate::services::hasher::Hasher;
use crate::services::mailer::Mailer;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

pub struct RegisterUser;

impl RegisterUser {
    pub async fn exec(
        db_manager: Arc<Mutex<DbManager>>,
        rep: impl UserRepository,
        hasher: HasherAdapter<impl Hasher>,
        tokenizer: TokenizerAdapter<impl Tokenizer>,
        mailer: MailerAdapter<impl Mailer>,
        templater: TemplaterAdapter<impl Templater>,
        template_name: &str,
        request_data: RequestData,
    ) -> Result<Uuid, AuthError>
    {
        let email_exists: bool = rep.email_exists(request_data.email()).await?;

        if email_exists {
            return Err(AuthError::Domain(DomainError::UserAlreadyExists));
        }

        let hashed_password = hasher.hash(request_data.password().to_string())?;
        let confirmation_token = tokenizer.generate()?;


        let user_data = UserData::new(
            request_data.email().to_string(),
            request_data.password().to_string(),
            request_data.password_confirmation().to_string(),
            hashed_password,
            confirmation_token,
        );
        let user = User::register(user_data.clone())
            .map_err(
                |e| AuthError::Domain(e)
            )?;

        let _ = rep.create(&user).await?;

        let mut body_data = HashMap::new();
        let url = format!(
            "http://localhost:8080/auth/confirm?email={}&token={}", user_data.email(), user_data.confirmation_token()
        );
        body_data.insert("url", url);

        let body = templater.render(template_name, body_data)?;

        let res = mailer.send(user.email().value().to_string(), "Confirmation email".to_string(), body).await;

        if let Err(e) = res {
            let mut guard = db_manager.lock().await;
            guard.rollback().await
                .map_err(|e|
                    AuthError::Infrastructure(
                        InfrastructureError::Transaction(e.to_string())
                    )
                )?;

            return Err(e);
        }

        let mut guard = db_manager.lock().await;
        guard.commit()
            .await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Transaction(e.to_string())
                )
            )?;

        Ok(user.id().clone())
    }
}
