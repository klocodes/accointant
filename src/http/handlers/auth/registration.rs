use actix_web::{post, HttpResponse, Responder, web::Json};
use actix_web::web::Data;
use serde::Deserialize;
use validator::Validate;

use crate::db::manager::db_manager::DbManager;
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::features::auth::application::command::register::{RegisterCommand};
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
async fn register(data: Json<RequestData>, state: Data<DbManager>) -> Result<impl Responder, Error> {
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

    let user_rep = DbUserRepository::new(state.get_ref().clone());
    let _ = RegisterCommand::exec(user_rep, data.into_inner()).await?;

    Ok(HttpResponse::Ok())
}