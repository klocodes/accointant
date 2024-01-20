/*use std::ops::Deref;
use actix_web::{post, HttpResponse, Responder, web::Json};
use actix_web::web::Data;
use serde::Deserialize;
use validator::Validate;
use crate::db::manager::db_manager::DbManager;
use crate::db::query_builder::QueryBuilder;

use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::errors::server::ServerErrors;
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
async fn register(data: Json<RequestData>, state: Data<dyn DbManager>) -> Result<impl Responder, Error> {
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

   /* let db_manager = state.as_ref().clone();
    let query_builder = QueryBuilder::new(db_manager);

    let user_rep = DbUserRepository::new(query_builder);

    let _ = RegisterCommand::exec(user_rep, data.into_inner())?;
*/
    Ok(HttpResponse::Ok())

}*/