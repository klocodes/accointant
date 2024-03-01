use std::collections::HashMap;
use async_trait::async_trait;
use thiserror::Error;

use crate::events::event::Event;
use crate::support::error::{FeatureError, SupportError};

pub trait Command {
    fn name() -> &'static str;
}

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&mut self, command: C) -> Result<Vec<Event>, FeatureError>;
}

pub struct CommandBus<C, H>
    where
        C: Command,
        H: CommandHandler<C>,
{
    handlers: HashMap<String, H>,
    _phantom: std::marker::PhantomData<C>,
}

impl<C, H> CommandBus<C, H>
    where
        C: Command,
        H: CommandHandler<C>,
{
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn register(&mut self, handler: H) {
        self.handlers.insert(C::name().to_string(), handler);
    }

    pub async fn dispatch(&mut self, command: C) -> Result<Vec<Event>, FeatureError>
    {
        let name = C::name();

        let handler = self.handlers.get_mut(name).ok_or_else(||
            FeatureError::Support(
                SupportError::CommandBus(
                    CommandBusError::HandlerNotFound(name.to_string())
                )
            )
        )?;

        handler.handle(command).await
    }
}

#[derive(Clone, Debug, Error)]
pub enum CommandBusError {
    #[error("Handler for command {0} not found")]
    HandlerNotFound(String),
}
