use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Path};
use serde::Deserialize;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::features::auth::application::request_confirmation_token::RequestConfirmationToken;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::templater::Templater;

#[derive(Debug, Deserialize)]
pub struct UserId(String);

#[post("/request-confirmation-token/{id}")]
async fn request(user_id: Path<UserId>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, Error> {
    let service_container  = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let user_rep = DbUserRepository::new(db_manager.clone(), serializer);

    let tokenizer = service_container.tokenizer();

    let mailer = service_container.mailer()?;
    let mailer_template_name = "confirm_registration";

    let mut templater = service_container.templater()?;
    templater.register(mailer_template_name, "mail/confirm_registration.hbs")?;

    let _ = RequestConfirmationToken::exec(
        db_manager,
        user_rep,
        tokenizer,
        mailer,
        templater,
        mailer_template_name,
        user_id.0.as_str()
    ).await?;

    Ok(HttpResponse::Ok())
}

