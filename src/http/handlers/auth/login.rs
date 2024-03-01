use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{HttpResponse, post, Responder};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use validator::Validate;
use crate::di::service_container::ServiceContainer;
use crate::features::auth::application::login_user::LoginUser;
use crate::features::auth::infrastructure::adapters::hasher_adapter::HasherAdapter;
use crate::features::auth::infrastructure::adapters::jwt_adapter::JwtServiceAdapter;
use crate::features::auth::infrastructure::db_user_repository::DbUserRepository;
use crate::http::error::HttpError;

#[derive(Deserialize, Validate)]
struct RequestData {
    #[validate(email)]
    email: String,

    password: String,
}

impl RequestData {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[post("/login")]
async fn login(data: Json<RequestData>, state: Data<Arc<ServiceContainer>>) -> Result<impl Responder, HttpError> {

    let service_container = state.into_inner();

    let db_manager = service_container.db_manager();
    let serializer = service_container.serializer();
    let rep = DbUserRepository::new(db_manager, serializer);

    let hasher = service_container.hasher();
    let hasher_adapter = HasherAdapter::new(hasher);
    let jwt_service = service_container.jwt_service();
    let jwt_service_adapter = JwtServiceAdapter::new(jwt_service);

    let login_user = LoginUser::new(
        data.email().to_string(),
        data.password().to_string(),
    );
    let token = login_user.exec(hasher_adapter, jwt_service_adapter, rep)
        .await
        .map_err(|e|
            HttpError::Feature(e)
        )?;

    let mut response = HashMap::new();
    response.insert("token", token);

    Ok(HttpResponse::Ok().json(response))
}