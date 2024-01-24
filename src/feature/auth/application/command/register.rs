use crate::bootstrap::app_context::TransactionManager;
use crate::db::db_transaction::DbTransaction;
use crate::errors::client::ClientErrors;
use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::feature::auth::domain::user::User;
use crate::feature::auth::domain::email::Email;
use crate::feature::auth::domain::password::Password;
use crate::feature::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::registration::RequestData;
use crate::service::mailer::Mailer;

pub struct RegisterCommand;

impl RegisterCommand {
    pub async fn exec<M>(mut transaction_manager: TransactionManager, rep: impl UserRepository, mailer: M, request_data: RequestData) -> Result<(), Error>
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


        let email = Email::new(request_data.email()).map_err(|e| {
            Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to create email: {}", e.to_string()).into())
            })
        })?;

        let password = Password::new(request_data.password()).map_err(|e| {
            Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to create password: {}", e.to_string()).into())
            })
        })?;

        let user = User::new(email.clone(), password.clone(), password)?;

        let _ = rep.create(&mut transaction_manager, &user).await?;

        let body = format!("Hello, {}! Your password is {}", user.email().value(), user.password().value());

        let res = mailer.send(user.email().value().to_string(), "Confirmation email".to_string(), body).await;

        if let Err(e) = res {
            return Err(e);
        }

        transaction_manager.commit().await?;

        Ok(())
    }
}