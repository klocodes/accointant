use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::{log_error, log_trace};
use crate::events::error::EventError;
use crate::features::auth::error::AuthError;
use crate::features::balance::error::BalanceError;
use crate::features::categories::error::CategoryError;
use crate::features::operations::error::OperationError;
use crate::features::tags::error::TagError;
use crate::support::error::FeatureError;

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

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized. {0}")]
    Unauthorized(String),

    #[error("Service container not found")]
    ServiceContainerNotFound,
}
impl ResponseError for HttpError {
    fn status_code(&self, ) -> StatusCode {
        match self {
            HttpError::Feature(feature_errors) => match feature_errors {
                FeatureError::Auth(auth_error) => match auth_error {
                    AuthError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    AuthError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                FeatureError::Balance(balance_error) => match balance_error {
                    BalanceError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    BalanceError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                FeatureError::Category(category_error) => match category_error {
                    CategoryError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    CategoryError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                FeatureError::Operation(operation_error) => match operation_error {
                    OperationError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    OperationError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                FeatureError::Tag(tag_error) => match tag_error {
                    TagError::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    TagError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },

                _ => StatusCode::INTERNAL_SERVER_ERROR,
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
