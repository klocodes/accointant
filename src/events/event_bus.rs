use async_trait::async_trait;
use crate::errors::Error;
use crate::events::event::Event;

#[async_trait]
pub trait EventBus: Send + Sync + 'static {

    async fn publish(&mut self, event: Event) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::server::ServerErrors::InternalServerError;

    pub struct MockEventBus {
        has_error: bool,
    }

    impl MockEventBus {
        pub fn new(has_error: bool) -> Self {
            Self {
                has_error
            }
        }
    }

    #[async_trait]
    impl EventBus for MockEventBus {
        async fn publish(&mut self, _event: Event) -> Result<(), Error> {
            if self.has_error {
                return Err(Error::Server(InternalServerError {
                    context: Some("Error".into())
                }));
            }

            Ok(())
        }
    }
}