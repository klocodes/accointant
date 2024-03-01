use std::sync::Arc;
use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde::Deserialize;
use uuid::Uuid;

use crate::di::service_container::ServiceContainer;
use crate::features::auth::application::confirm::ConfirmRegistration;
use crate::features::auth::infrastructure::adapters::tokenizer_adapter::TokenizerAdapter;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::http::error::HttpError;

#[derive(Deserialize)]
pub struct RequestData {
    id: Uuid,
    token: String,
}

impl RequestData {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[get("/confirm")]
async fn confirm(request_data: Query<RequestData>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, HttpError> {
    let service_container = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();

    let rep = DbUserRepository::new(db_manager, serializer);
    let tokenizer = service_container.tokenizer();
    let tokenizer_adapter = TokenizerAdapter::new(tokenizer);

    let use_case = ConfirmRegistration::new(
        request_data.id().clone(),
        request_data.token().to_string(),
    );
    use_case.exec(rep, tokenizer_adapter)
        .await
        .map_err(|e|
            HttpError::Feature(e)
        )?;

    Ok(HttpResponse::Ok())
}