use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use serde::Deserialize;
use crate::bootstrap::app_context::{AppContext, TransactionManager};
use crate::errors::Error;
use crate::features::auth::application::request_confirmation_token::RequestConfirmationToken;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

#[derive(Debug, Deserialize)]
pub struct UserId(String);
#[post("/auth/request-confirmation-token/{id}")]
async fn request(user_id: Path<UserId>, state: Data<AppContext>) -> Result<impl Responder, Error> {
    let app_context = state.get_ref().clone();

    let user_rep = DbUserRepository::new(app_context.clone());
    let transaction_manager = TransactionManager::new();

    let tokenizer = Tokenizer::new();

    let mailer = app_context.get_mailer().clone();
    let template_name = "confirm_registration";
    let templater = Templater::new(template_name, "html/mail/confirm_registration.hbs")?;

    let _ = RequestConfirmationToken::exec(
        transaction_manager,
        user_rep,
        tokenizer,
        mailer,
        templater,
        template_name,
        user_id.0.as_str()
    ).await?;

    Ok(HttpResponse::Ok())
}

