use async_trait::async_trait;
use crate::db::transaction::container::TransactionContainer;
use crate::errors::Error;
use crate::features::operations::application::commands::create_operation::command::CreateOperationCommand;
use crate::features::operations::domain::operation::Operation;
use crate::features::operations::domain::operation_event::OperationEvent;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::services::serializer::Serializer;
use crate::support::command_bus::CommandHandler;

#[derive(Debug)]
pub struct CreateOperationCommandHandler<'a, R>
    where
        R: OperationRepository,
{
    rep: R,
    transaction_container: TransactionContainer<'a>,
}

impl<R> CreateOperationCommandHandler<'_, R,>
    where
        R: OperationRepository,
{
    pub fn new(rep: R, transaction_container: TransactionContainer<'static>) -> Self {
        Self { rep, transaction_container }
    }
}

#[async_trait]
impl<R> CommandHandler<CreateOperationCommand> for CreateOperationCommandHandler<'_, R>
    where
        R: OperationRepository + Send + Sync,
{
    async fn handle(&self, command: CreateOperationCommand) -> Result<(), Error> {
        let events = Operation::handle_creation(command)?;

        for event in events {
            match event {
                OperationEvent::OperationCreated(data) => {
                    self.rep.persist_operation_created_event(data).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
