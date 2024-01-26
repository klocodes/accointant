use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use serde::Deserialize;
use crate::bootstrap::app_context::{AppContext, TransactionManager};
use crate::config::structs::templater;
use crate::db::connection::manager::ConnectionManager;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::features::auth::application::request_confirmation_token::RequestConfirmationToken;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

#[derive(Debug, Deserialize)]
pub struct UserId(String);
#[post("/auth/request-confirmation-token/{id}")]
async fn request(user_id: Path<UserId>, state: Data<(AppContext, ServiceContainer)>) -> Result<impl Responder, Error> {
    let (app_context, service_container) = state.get_ref().clone();

    let user_rep = DbUserRepository::new(app_context.clone(), service_container.serializer());
    let transaction_manager = TransactionManager::new();

    let tokenizer = service_container.tokenizer();

    let mailer = app_context.get_mailer().clone();
    let mailer_template_name = "confirm_registration";

    let mut templater = service_container.templater()?;
    templater.register(mailer_template_name, "mail/confirm_registration.hbs")?;

    let _ = RequestConfirmationToken::exec(
        transaction_manager,
        user_rep,
        tokenizer,
        mailer,
        templater,
        mailer_template_name,
        user_id.0.as_str()
    ).await?;

    Ok(HttpResponse::Ok())
}

