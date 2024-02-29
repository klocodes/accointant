use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use serde::Deserialize;
use uuid::Uuid;
use crate::di::service_container::ServiceContainer;
use crate::errors::error::AppError;
use crate::features::auth::application::request_confirmation_token::RequestConfirmationToken;
use crate::features::auth::infrastructure::adapters::mailer_adapter::MailerAdapter;
use crate::features::auth::infrastructure::adapters::templater_adapter::TemplaterAdapter;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::templater::Templater;

#[derive(Debug, Deserialize)]
pub struct UserId(pub Uuid);

#[post("/request-confirmation-token/{id}")]
async fn request(user_id: Path<UserId>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, AppError> {
    let service_container = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let user_rep = DbUserRepository::new(db_manager.clone(), serializer);

    let tokenizer = service_container.tokenizer();
    let tokenizer = TokenizerAdapter::new(tokenizer);

    let mailer = service_container.mailer();
    let mailer = MailerAdapter::new(mailer);
    let mailer_template_name = "confirm_registration";

    let templater = service_container.templater();
    let mut templater = TemplaterAdapter::new(templater);
    templater.register(mailer_template_name, "mail/confirm_registration.hbs")
        .map_err(|e|
            AppError::Auth(e)
        )?;

    let _ = RequestConfirmationToken::exec(
        db_manager,
        user_rep,
        tokenizer,
        mailer,
        templater,
        mailer_template_name,
        user_id.0,
    ).await.map_err(
        |e| AppError::Auth(e)
    )?;

    Ok(HttpResponse::Ok())
}

