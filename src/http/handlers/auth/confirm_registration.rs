use std::sync::Arc;
use actix_web::{get, HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde::Deserialize;
use uuid::Uuid;

use crate::di::service_container::ServiceContainer;
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::features::auth::application::confirm::ConfirmRegistration;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;

#[derive(Deserialize)]
pub struct RequestData {
    id: String,
    token: String,
}

impl RequestData {
    pub fn id(&self) -> Result<Uuid, Error> {
        Uuid::parse_str(&self.id).map_err(|e|
            Error::Client(
                BadRequest {
                    message: Some(
                        format!("Failed to parse id: {}", e.to_string()).into()
                    )
                }
            )
        )
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[get("/confirm")]
async fn confirm(request_data: Query<RequestData>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, Error> {
    let service_container = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();

    let rep = DbUserRepository::new(db_manager, serializer);
    let tokenizer = service_container.tokenizer();

    let _ = ConfirmRegistration::exec(rep, tokenizer, request_data.into_inner()).await?;

    Ok(HttpResponse::Ok())
}