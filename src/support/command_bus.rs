use std::collections::HashMap;
use async_trait::async_trait;

use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

pub trait Command {
    fn name() -> &'static str;
}

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(&self, command: C) -> Result<(), Error>;
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

    pub async fn dispatch(&self, command: C) -> Result<(), Error> {
        let name = C::name();

        let handler = self.handlers.get(name).ok_or(
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Handler for command {} not found", name).into()
                    )
                }
            )
        )?;

        let _ = handler.handle(command).await?;

        Ok(())
    }
}