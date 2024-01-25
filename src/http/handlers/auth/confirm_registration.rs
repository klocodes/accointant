use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde::Deserialize;

use crate::bootstrap::app_context::AppContext;
use crate::errors::Error;
use crate::features::auth::application::command::confirm::ConfirmCommand;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::tokenizer::Tokenizer;

#[derive(Deserialize)]
pub struct RequestData {
    email: String,
    token: String,
}

impl RequestData {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[get("/confirm")]
async fn confirm(request_data: Query<RequestData>, state: Data<AppContext>) -> Result<impl Responder, Error> {
    let app_context = state.as_ref().clone();

    let rep = DbUserRepository::new(app_context.clone());
    let tokenizer = Tokenizer::new();

    let _ = ConfirmCommand::exec(rep, tokenizer, request_data.into_inner()).await?;

    Ok(HttpResponse::Ok())
}