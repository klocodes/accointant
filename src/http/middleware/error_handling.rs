use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready}, Error as ActixError, http, ResponseError};
use actix_web::body::{BoxBody, MessageBody};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use crate::http::error_handler::from_status;

pub struct ErrorHandling;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandling
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
        S::Future: 'static,
        B: MessageBody + 'static, // Добавлено ограничение MessageBody
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type Transform = ErrorHandlingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlingMiddleware { service }))
    }
}

pub struct ErrorHandlingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlingMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
        S::Future: 'static,
        B: MessageBody + 'static, // Добавлено ограничение MessageBody
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await?;

            if let Some(error) = result.response().error() {
                let custom_error = from_status(&error);
                let custom_response = custom_error.error_response();

                // Изменение тела ответа с использованием map_body
                let new_response = result.map_body(|head, _body| {
                    head.headers_mut().insert(
                        http::header::CONTENT_TYPE,
                        http::header::HeaderValue::from_static("application/json"),
                    );

                    BoxBody::new(custom_response.into_body())
                });

                Ok(new_response)
            } else {
                // Приведение тела к типу BoxBody
                let new_response = result.map_body(|_head, body| {
                    BoxBody::new(body)
                });

                Ok(new_response)
            }
        })
    }
}