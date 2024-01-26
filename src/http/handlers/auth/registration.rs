use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use validator::Validate;

use crate::db::connection::manager::ConnectionManager;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::features::auth::application::register_user::RegisterUser;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
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

#[post("/register")]
async fn register(data: Json<RequestData>, state: Data<ServiceContainer>) -> Result<impl Responder, Error> {
    if let Err(e) = data.validate() {
        return Err(Error::Client(ClientErrors::BadRequest { message: Some(e.to_string().into()) }));
    }

    let service_container = state.as_ref().clone();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let user_rep = DbUserRepository::new(db_manager.clone(), serializer);

    let transaction_container = db_manager.transaction_container()?;

    let tokenizer = service_container.tokenizer();
    let hasher = BcryptHasher::new();

    let mailer = service_container.mailer()?;
    let mailer_template_name = "confirm_registration";
    let mut templater = service_container.templater()?;
    templater.register(mailer_template_name, "confirm_registration.hbs")?;

    let _ = RegisterUser::exec(
        transaction_container,
        user_rep,
        hasher,
        tokenizer,
        mailer,
        templater,
        mailer_template_name,
        data.into_inner(),
    ).await?;

    Ok(HttpResponse::Ok())
}