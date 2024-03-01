use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::errors::Error;
use crate::errors::client::ClientErrors;
use crate::errors::network::NetworkErrors;
use crate::errors::server::ServerErrors;
use crate::{log_error, log_trace};
use crate::events::error::EventError;
use crate::features::auth::error::AuthError;
use crate::features::operations::error::OperationError;
use crate::support::error::FeatureError;

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Client(client_errors) => {
                match client_errors {
                    ClientErrors::BadRequest { .. } => StatusCode::BAD_REQUEST,
                    ClientErrors::DomainError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
                    ClientErrors::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
                    ClientErrors::PaymentRequired { .. } => StatusCode::PAYMENT_REQUIRED,
                    ClientErrors::NotFound { .. } => StatusCode::NOT_FOUND,
                }
            }
            Error::Network(network_errors) => {
                match network_errors {
                    NetworkErrors::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
                    NetworkErrors::NotAcceptable { .. } => StatusCode::NOT_ACCEPTABLE,
                    NetworkErrors::RequestTimeout { .. } => StatusCode::REQUEST_TIMEOUT,
                    NetworkErrors::UriTooLong { .. } => StatusCode::URI_TOO_LONG,
                    NetworkErrors::UnsupportedMediaType { .. } => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    NetworkErrors::RangeNotSatisfiable { .. } => StatusCode::RANGE_NOT_SATISFIABLE,
                    NetworkErrors::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,
                    NetworkErrors::RequestHeaderFieldsTooLarge { .. } => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    NetworkErrors::NetworkAuthenticationRequired { .. } => StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                }
            }
            Error::Server(server_errors) => {
                match server_errors {
                    ServerErrors::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                    ServerErrors::NotImplemented { .. } => StatusCode::NOT_IMPLEMENTED,
                    ServerErrors::BadGateway { .. } => StatusCode::BAD_GATEWAY,
                    ServerErrors::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
                    ServerErrors::GatewayTimeout { .. } => StatusCode::GATEWAY_TIMEOUT,
                    ServerErrors::HttpVersionNotSupported { .. } => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    ServerErrors::VariantAlsoNegotiates { .. } => StatusCode::VARIANT_ALSO_NEGOTIATES,
                    ServerErrors::InsufficientStorage { .. } => StatusCode::INSUFFICIENT_STORAGE,
                    ServerErrors::LoopDetected { .. } => StatusCode::LOOP_DETECTED,
                    ServerErrors::NotExtended { .. } => StatusCode::NOT_EXTENDED,
                    ServerErrors::Forbidden { .. } => StatusCode::FORBIDDEN,
                    ServerErrors::Gone { .. } => StatusCode::GONE,
                    ServerErrors::LengthRequired { .. } => StatusCode::LENGTH_REQUIRED,
                    ServerErrors::PayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
                    ServerErrors::Conflict { .. } => StatusCode::CONFLICT,
                    ServerErrors::PreconditionFailed { .. } => StatusCode::PRECONDITION_FAILED,
                    ServerErrors::ExpectationFailed { .. } => StatusCode::EXPECTATION_FAILED,
                    ServerErrors::ImATeapot { .. } => StatusCode::IM_A_TEAPOT,
                    ServerErrors::MisdirectedRequest { .. } => StatusCode::MISDIRECTED_REQUEST,
                    ServerErrors::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
                    ServerErrors::Locked { .. } => StatusCode::LOCKED,
                    ServerErrors::FailedDependency { .. } => StatusCode::FAILED_DEPENDENCY,
                    ServerErrors::UpgradeRequired { .. } => StatusCode::UPGRADE_REQUIRED,
                    ServerErrors::PreconditionRequired { .. } => StatusCode::PRECONDITION_REQUIRED,
                    ServerErrors::UnavailableForLegalReasons { .. } => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                }
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        log_error!("Error response: {}", error_message);
        log_trace!("Trace response: {}", error_message);

        HttpResponse::build(status_code)
            .json(serde_json::json!({
                "error": error_message
            }))
    }
}


#[derive(Clone, Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Feature error. {0}")]
    Feature(FeatureError),

    #[error("Request validation error. {0}")]
    RequestValidation(String),

    #[error("Event error. {0}")]
    Event(EventError),

    #[error("Service error. {0}")]
    Service(String),
}
impl ResponseError for HttpError {
    fn status_code(&self, ) -> StatusCode {
        match self {
            HttpError::Feature(feature_errors) => match feature_errors {
                FeatureError::Auth(auth_error) => match auth_error {
                    AuthError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    AuthError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                FeatureError::Operation(operation_error) => match operation_error {
                    OperationError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    OperationError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },
            },
            HttpError::RequestValidation(_) => StatusCode::BAD_REQUEST,
            HttpError::Service(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        log_error!("Error response: {}", error_message);
        log_trace!("Trace response: {}", error_message);

        HttpResponse::build(status_code)
            .json(serde_json::json!({
                "error": error_message
            }))
    }
}