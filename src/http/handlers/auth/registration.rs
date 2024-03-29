use std::sync::Arc;
use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::di::service_container::ServiceContainer;
use crate::features::auth::application::register_user::RegisterUser;
use crate::features::auth::infrastructure::adapters::hasher_adapter::HasherAdapter;
use crate::features::auth::infrastructure::adapters::mailer_adapter::MailerAdapter;
use crate::features::auth::infrastructure::adapters::templater_adapter::TemplaterAdapter;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::http::error::HttpError;
use crate::services::hasher::BcryptHasher;
use crate::services::templater::Templater;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    pub id: Uuid,
}

#[post("/register")]
async fn register(data: Json<RequestData>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, HttpError> {
    if let Err(e) = data.validate() {
        return Err(
            HttpError::RequestValidation(e.to_string())
        );
    }

    let service_container = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let user_rep = DbUserRepository::new(db_manager.clone(), serializer);

    let tokenizer = service_container.tokenizer();
    let tokenizer_adapter = TokenizerAdapter::new(tokenizer);
    let hasher = BcryptHasher::new();
    let hasher_adapter = HasherAdapter::new(hasher);

    let mailer = service_container.mailer();
    let mailer_adapter = MailerAdapter::new(mailer);
    let mailer_template_name = "confirm_registration";

    let mut templater = service_container.templater();
    let templater_adapter = TemplaterAdapter::new(templater.clone());
    templater.register(mailer_template_name, "mail/confirm_registration.hbs")
        .map_err(|e| HttpError::Service(e.to_string()))?;

    let user_id = RegisterUser::exec(
        db_manager,
        user_rep,
        hasher_adapter,
        tokenizer_adapter,
        mailer_adapter,
        templater_adapter,
        mailer_template_name,
        data.into_inner(),
    ).await.map_err(
        |e| HttpError::Feature(e)
    )?;

    Ok(
        HttpResponse::Ok().json(ResponseData {
            id: user_id
        })
    )
}