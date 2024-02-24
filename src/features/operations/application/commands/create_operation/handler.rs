use async_trait::async_trait;
use crate::errors::Error;
use crate::events::event::Event;
use crate::features::operations::application::commands::create_operation::command::CreateOperationCommand;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::features::operations::domain::operation::Operation;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::support::command_bus::CommandHandler;

#[derive(Debug)]
pub struct CreateOperationCommandHandler<R>
    where
        R: OperationRepository + Send + Sync,
{
    rep: R,
}

impl<R> CreateOperationCommandHandler<R>
    where
        R: OperationRepository + Send + Sync,
{
    pub fn new(rep: R) -> Self {
        Self {
            rep,
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[async_trait]
impl<R> CommandHandler<CreateOperationCommand> for CreateOperationCommandHandler<R>
    where
        R: OperationRepository + Send + Sync,
{
    async fn handle(&mut self, command: CreateOperationCommand) -> Result<Vec<Event>, Error> {
        let mut events = vec![];
        let operation_events = Operation::handle_creation(command)?;

        for event in operation_events {
            match event {
                OperationEvent::OperationCreated(ref operation_created) => {
                    self.rep.persist_operation_created_event(operation_created.clone()).await?;

                    true
                }
                _ => false
            };

           events.push(Event::OperationEvent(event));
        }

        println!("{:?}", events);

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::operations::domain::operation_repository::MockOperationRepository;
    use crate::features::shared::id::Id;
    use super::*;

    #[tokio::test]
    async fn test_handle_success() {
        let rep = MockOperationRepository::new(false);

        let command = command_fixture();
        let mut handler = CreateOperationCommandHandler::new(rep);

        let res = handler.handle(command).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_handle_error() {
        let rep = MockOperationRepository::new(true);

        let command = command_fixture();
        let mut handler = CreateOperationCommandHandler::new(rep);

        let res = handler.handle(command).await;

        assert!(res.is_err());
    }

    fn command_fixture() -> CreateOperationCommand {
        CreateOperationCommand::new(
            String::from("Income"),
            Id::generate(),
            Some(Id::generate()),
            String::from("Food"),
            100.0,
            String::from("USD"),
            100.0,
            1.0,
            String::from("Grocery Shopping"),
            vec![],
        )
    }
}