use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use validator::Validate;

use crate::bootstrap::app_context::{AppContext, TransactionManager};
use crate::db::connection::manager::ConnectionManager;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::features::auth::application::register_user::RegisterUser;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::services::hasher::BcryptHasher;
use crate::services::templater::Templater;
use crate::services::tokenizer::Tokenizer;

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
async fn register(data: Json<RequestData>, state: Data<(AppContext, ServiceContainer)>) -> Result<impl Responder, Error> {
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

    let (app_context, service_container)  = state.as_ref().clone();

    let user_rep = DbUserRepository::new(app_context.clone(), service_container.serializer());
    let transaction_manager = TransactionManager::new();

    let tokenizer = service_container.tokenizer();
    let hasher = BcryptHasher::new();

    let mailer = app_context.get_mailer().clone();

    let template_name = "";
    let mut templater = service_container.templater()?;
    templater.register(template_name, "confirm_registration.hbs")?;

    let _ = RegisterUser::exec(
        transaction_manager,
        user_rep,
        hasher,
        tokenizer,
        mailer,
        templater,
        template_name,
        data.into_inner(),
    ).await?;

    Ok(HttpResponse::Ok())
}