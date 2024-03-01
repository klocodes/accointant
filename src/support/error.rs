use thiserror::Error;
use crate::features::auth::error::AuthError;
use crate::features::balance::error::BalanceError;
use crate::features::categories::error::CategoryError;
use crate::features::operations::error::OperationError;
use crate::features::tags::error::TagError;
use crate::support::command_bus::CommandBusError;

#[derive(Debug, Clone, Error)]
pub enum FeatureError {
    #[error("Support error. {0}")]
    Support(SupportError),

    #[error("Auth bounded context error. {0}")]
    Auth(AuthError),

    #[error("Balance bounded context error. {0}")]
    Balance(BalanceError),

    #[error("Category bounded context error. {0}")]
    Category(CategoryError),

    #[error("Operation bounded context error. {0}")]
    Operation(OperationError),

    #[error("Tag bounded context error. {0}")]
    Tag(TagError),
}

#[derive(Debug, Clone, Error)]
pub enum SupportError {
    #[error("Command bus error. {0}")]
    CommandBus(CommandBusError),

    #[error("Data mapper error. {0}")]
    DataMapper(String),

    #[error("Id error. {0}")]
    Id(String),
}