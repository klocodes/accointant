use crate::errors::client::ClientErrors;
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::http::handlers::auth::registration::RequestData;

pub struct RegisterCommand;

impl RegisterCommand {
    pub async fn exec(rep: impl UserRepository, request_data: RequestData) -> Result<(), Error> {
        let user: Option<User> = rep.find_by_email(request_data.email())
            .await?;

        if let Some(_) = user {
            return Err(Error::Client(ClientErrors::BadRequest {
                message: Some(
                    format!(
                        "User with email {} already exists", request_data.email()
                    ).into()
                )
            }));
        }


        let email = Email::new(request_data.email()).unwrap();
        let password = Password::new(request_data.password()).unwrap();

        let user = User::new(email, password.clone(), password);

        Ok(())
    }
}