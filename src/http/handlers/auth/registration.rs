use actix_web::{post, HttpResponse, Responder, web::Json};
use actix_web::web::Data;
use serde::Deserialize;
use validator::Validate;

use crate::bootstrap::app_context::{AppContext, TransactionManager};
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::feature::auth::application::command::register::RegisterCommand;
use crate::feature::auth::infrastructure::db_user_repository::DbUserRepository;

#[derive(Deserialize, Validate)]
pub struct RequestData {
    #[validate(email)]
    email: String,

    #[validate(length(min = 6))]
    password: String,

    #[validate(must_match = "password")]
    password_confirmation: String,
}

impl RequestData {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn password_confirmation(&self) -> &str {
        &self.password_confirmation
    }
}

#[post("/register")]
async fn register(data: Json<RequestData>, state: Data<AppContext>) -> Result<impl Responder, Error>
{
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

    let app_context = state.as_ref().clone();

    let user_rep = DbUserRepository::new(app_context.clone());
    let transaction_manager = TransactionManager::new();
    let mailer = app_context.get_mailer().clone();

    let _ = RegisterCommand::exec(transaction_manager, user_rep, mailer, data.into_inner()).await?;

    Ok(HttpResponse::Ok())

}