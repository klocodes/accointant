use actix_web::{post, HttpResponse, Responder, web::Json};
use actix_web::web::Data;
use serde::Deserialize;
use validator::Validate;

use crate::bootstrap::app_context::AppContext;
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::features::auth::application::command::register::RegisterCommand;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;

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

    let user_rep = DbUserRepository::new(app_context);

    let _ = RegisterCommand::exec(user_rep, data.into_inner()).await?;

    Ok(HttpResponse::Ok())

}