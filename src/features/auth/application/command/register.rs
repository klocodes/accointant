use crate::errors::client::ClientErrors;
use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::registration::RequestData;

pub struct RegisterCommand;

impl RegisterCommand {
    pub async fn exec(rep: impl UserRepository, request_data: RequestData) -> Result<(), Error> {
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

        let user = User::new(email, password.clone(), password)?;

        rep.create(&user).await?;

        Ok(())
    }
}