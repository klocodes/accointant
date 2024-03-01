use std::future::{ready, Ready};
use std::sync::Arc;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::body::BoxBody;
use actix_web::web::Data;
use futures_util::future::LocalBoxFuture;

use crate::di::service_container::ServiceContainer;
use crate::http::error::HttpError;
use crate::services::jwt::JwtService;

pub struct CheckAuth;

impl<S> Transform<S, ServiceRequest> for CheckAuth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = CheckAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthMiddleware { service }))
    }
}

pub struct CheckAuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CheckAuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service_container = req.app_data::<Data<Arc<ServiceContainer>>>();

        if service_container.is_none() {
            let error_response = req.error_response(HttpError::ServiceContainerNotFound);

            return Box::pin(async move { Ok(error_response) });
        }

        let service_container = service_container.unwrap().as_ref().clone();
        let jwt_service = service_container.jwt_service();

        let token = req.headers().get("Authorization");
        if token.is_none() {
            let error = HttpError::Unauthorized("Token not found".to_string());

            let error_response = req.error_response(error);

            return Box::pin(async move { Ok(error_response) });
        }

        let token = token.unwrap().to_str().unwrap();
        let parts: Vec<&str> = token.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            let error = HttpError::Unauthorized("Invalid token".to_string());

            let error_response = req.error_response(error);

            return Box::pin(async move { Ok(error_response) });
        }

        let token = parts[1];
        println!("token: {}", token);


        let verification = jwt_service.verify(token);

        if verification.is_err() {
            let error = HttpError::Unauthorized(verification.unwrap_err().to_string());

            let error_response = req.error_response(error);

            return Box::pin(async move { Ok(error_response) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}