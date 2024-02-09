use async_trait::async_trait;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::features::operations::application::commands::create_operation::command::CreateOperationCommand;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::features::operations::domain::operation::Operation;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::support::command_bus::CommandHandler;

#[derive(Debug)]
pub struct CreateOperationCommandHandler<R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    rep: R,
    db_manager: DbManager,
    event_bus: EB,
}

impl<R, EB> CreateOperationCommandHandler<R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    pub fn new(db_manager: DbManager, rep: R, event_bus: EB) -> Self {
        Self {
            rep,
            db_manager,
            event_bus,
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[async_trait]
impl<R, EB> CommandHandler<CreateOperationCommand> for CreateOperationCommandHandler<R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    async fn handle(&mut self, command: CreateOperationCommand) -> Result<(), Error> {
        let events = Operation::handle_creation(command)?;

        for event in events {
            let has_operation_created = match event {
                OperationEvent::OperationCreated(ref operation_created) => {
                    self.rep.persist_operation_created_event(operation_created.clone()).await?;

                    true
                }
                _ => false
            };

            let res = self.event_bus.publish(Event::OperationEvent(event.clone())).await;

            if has_operation_created {
                match res {
                    Ok(_) => {
                        self.db_manager.commit().await?;
                    }
                    Err(e) => {
                        self.db_manager.rollback().await?;

                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::manager::MockManager;
    use crate::events::event_bus::MockEventBus;
    use crate::features::operations::domain::operation_repository::MockOperationRepository;
    use crate::features::shared::id::Id;
    use super::*;

    #[tokio::test]
    async fn test_handle_success() {
        let rep = MockOperationRepository::new(false);
        let event_bus = MockEventBus::new(false);
        let db_manager = DbManager::Mock(MockManager::new(false));

        let command = command_fixture();
        let mut handler = CreateOperationCommandHandler::new(db_manager, rep, event_bus);

        let res = handler.handle(command).await;

        assert!(res.is_ok());
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

    #[tokio::test]
    async fn test_handle_error() {
        let rep = MockOperationRepository::new(true);
        let event_bus = MockEventBus::new(true);
        let db_manager = DbManager::Mock(MockManager::new(true));

        let command = command_fixture();
        let mut handler = CreateOperationCommandHandler::new(db_manager, rep, event_bus);

        let res = handler.handle(command).await;

        assert!(res.is_err());
    }
}