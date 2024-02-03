use async_trait::async_trait;
use crate::db::transaction::container::TransactionContainer;
use crate::db::transaction::manager::TransactionManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::events::event::Event;
use crate::events::event_bus::EventBus;
use crate::features::operations::application::commands::create_operation::command::CreateOperationCommand;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::features::operations::domain::operation::Operation;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::support::command_bus::CommandHandler;

#[derive(Debug)]
pub struct CreateOperationCommandHandler<'a, R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    rep: R,
    transaction_container: Option<TransactionContainer<'a>>,
    event_bus: EB,
}

impl<R, EB> CreateOperationCommandHandler<'_, R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    pub fn new(rep: R, transaction_container: TransactionContainer<'static>, event_bus: EB) -> Self {
        Self {
            rep,
            transaction_container: Some(transaction_container),
            event_bus
        }
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[async_trait]
impl<R, EB> CommandHandler<CreateOperationCommand> for CreateOperationCommandHandler<'_, R, EB>
    where
        R: OperationRepository + Send + Sync,
        EB: EventBus,
{
    async fn handle(&mut self, command: CreateOperationCommand) -> Result<(), Error> {
        let events = Operation::handle_creation(command)?;

        for event in events {
            let has_operation_created = match event {
                OperationEvent::OperationCreated(ref operation_created) => {
                    let tc = self.transaction_container.as_mut().ok_or(
                        Error::Server(
                            InternalServerError {
                                context: Some(
                                    "Transaction container not found".into()
                                )
                            }
                        )
                    )?;

                    self.rep.persist_operation_created_event(tc, operation_created.clone()).await?;

                    true
                }
                _ => false
            };

            let res = self.event_bus.publish(Event::OperationEvent(event.clone())).await;

            if has_operation_created {
                let mut tc = self.transaction_container.take().ok_or(
                    Error::Server(
                        InternalServerError {
                            context: Some(
                                "Transaction container not found".into()
                            )
                        }
                    )
                )?;

                match res {
                    Ok(_) => {
                        tc.take_manager().commit().await?;
                    }
                    Err(e) => {
                        tc.take_manager().rollback().await?;

                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}