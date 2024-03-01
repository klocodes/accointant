use chrono::{Utc};
use crate::features::operations::application::commands::create_operation::command::CreateOperationCommand;
use crate::features::operations::domain::amount::Amount;
use crate::features::operations::domain::currency::Currency;
use crate::features::operations::domain::error::DomainError;
use crate::features::operations::domain::events::category_creation_requested::CategoryCreationRequested;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::domain::events::tag_creation_requested::TagCreationRequested;
use crate::features::operations::domain::events::operation_event::OperationEvent;
use crate::features::operations::domain::kind::Kind;
use crate::support::id::Id;

pub struct Operation {
    id: Id,
    user_id: Id,
    kind: Kind,
    category_id: Id,
    amount: Amount,
    currency: Currency,
    currency_amount: Amount,
    rate: Amount,
    label: String,
    tags: Vec<Id>,
}

impl Operation {
    pub fn handle_creation(command: CreateOperationCommand) -> Result<Vec<OperationEvent>, DomainError> {
        let mut events: Vec<OperationEvent> = vec![];

        let operation_id = Id::new(Id::generate());
        let now = Utc::now();
        let user_id = Id::new(command.user_id().clone());
        let kind = Kind::new(command.kind())?;
        let amount = Amount::new(command.amount())?;
        let currency_amount = Amount::new(command.currency_amount())?;
        let rate = Amount::new(command.rate())?;

        if amount.value() != currency_amount.value() * rate.value() {
            return Err(
                DomainError::InvalidAmount(
                    format!("Amount {} is not equal to currency amount {} by rate {}", amount.value(), currency_amount.value(), rate.value())
                )
            );
        }

        let category_id = match command.category_id() {
            Some(id) => Id::new(id.clone()),
            None => {
                let category_id = Id::new(Id::generate());
                let category_created = OperationEvent::CategoryCreationRequested(
                    CategoryCreationRequested::new(
                        Id::new(Id::generate()),
                        operation_id.clone(),
                        user_id.clone(),
                        category_id.clone(),
                        command.category_name().to_string(),
                    )
                );

                events.push(category_created);

                category_id
            }
        };

        let currency = Currency::new(command.currency())?;
        let label = command.label().to_string();

        let mut tags: Vec<Id> = vec![];

        for tag in command.tags() {
            if tag.id().is_none() {
                let tag_id = Id::new(Id::generate());
                let tag_creation_requested = OperationEvent::TagCreationRequested(
                    TagCreationRequested::new(
                        Id::new(Id::generate()),
                        operation_id.clone(),
                        user_id.clone(),
                        tag_id.clone(),
                        tag.name().to_string(),
                    )
                );

                tags.push(tag_id);
                events.push(tag_creation_requested);
            } else {
                tags.push(Id::new(tag.id().unwrap().clone()));
            }
        }

        let operation = Self {
            id: operation_id,
            user_id: user_id.clone(),
            kind,
            category_id: category_id.clone(),
            amount,
            currency,
            currency_amount,
            rate,
            label,
            tags,
        };

        let operation_created = OperationEvent::OperationCreated(
            OperationCreated::new(
                Id::new(Id::generate()),
                operation.id().clone(),
                user_id.clone(),
                operation.kind().clone(),
                operation.category_id().clone(),
                operation.amount().clone(),
                operation.currency_amount().clone(),
                operation.currency().clone(),
                operation.rate().clone(),
                operation.label().to_string(),
                operation.tag_ids().to_vec(),
                now,
            )
        );

        events.push(operation_created);

        Ok(events)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn category_id(&self) -> &Id {
        &self.category_id
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn currency_amount(&self) -> &Amount {
        &self.currency_amount
    }

    pub fn rate(&self) -> &Amount {
        &self.rate
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn tag_ids(&self) -> &[Id] {
        &self.tags
    }
}

#[cfg(test)]
mod operation_creation_tests {
    use uuid::Uuid;
    use super::*;
    use crate::features::operations::application::commands::create_operation::command::{CreateOperationCommand, TagData};

    #[test]
    fn test_operation_creation_with_existing_category_and_tags() {
        // Create command
        let command = create_operation_command_fixture(true, true, false);

        let result = Operation::handle_creation(command.clone());
        assert!(result.is_ok());

        let events = result.unwrap();

        // Event must be one
        assert_eq!(events.len(), 1);
        let event = events.get(0);
        assert!(event.is_some());
        let event = event.unwrap();

        // Event must be OperationCreated
        match event {
            OperationEvent::OperationCreated(data) => {
                assert_operation_created_event(data.clone(), command.clone());

                assert_eq!(data.payload().category_id().value(), command.category_id().unwrap()); // Проверка соответствия category_id
                assert_eq!(data.payload().tag_ids().len(), 2);
                assert!(data.payload().tag_ids().contains(&Id::new(command.tags()[0].id().unwrap())) && data.payload().tag_ids().contains(&Id::new(command.tags()[1].id().unwrap())));
            }
            _ => {
                panic!("Unexpected event type");
            }
        }
    }

    #[test]
    pub fn test_operation_creation_with_new_category() {
        let command = create_operation_command_fixture(false, true, false);

        let result = Operation::handle_creation(command.clone());
        assert!(result.is_ok());

        // Event must be two (CategoryCreationRequested and OperationCreated)
        let events = result.unwrap();
        assert_eq!(events.len(), 2);
        let category_creation_requested = events.get(0);
        assert!(category_creation_requested.is_some());

        match category_creation_requested {
            Some(OperationEvent::CategoryCreationRequested(data)) => {
                assert_category_creation_requested_event(data.clone(), command.clone());
            }
            _ => {
                panic!("Unexpected event type");
            }
        }

        let operation_created = events.get(1);
        assert!(operation_created.is_some());
        let event = operation_created.unwrap();
        match &operation_created {
            Some(OperationEvent::OperationCreated(data)) => {
                assert_operation_created_event(data.clone(), command.clone());

                assert_eq!(data.payload().category_id().to_string().len(), 36);
                assert_eq!(data.payload().tag_ids().len(), 2);
                assert!(data.payload().tag_ids().contains(&Id::new(command.tags()[0].id().unwrap())) && data.payload().tag_ids().contains(&Id::new(command.tags()[1].id().unwrap())));
            }
            _ => {
                panic!("Unexpected event type");
            }
        }
    }

    #[test]
    pub fn test_operation_creation_with_new_tags() {
        let command = create_operation_command_fixture(true, true, true);

        let result = Operation::handle_creation(command.clone());
        assert!(result.is_ok());

        // Event must be three (TagCreationRequested, TagCreationRequested and OperationCreated)
        let events = result.unwrap();
        assert_eq!(events.len(), 3);

        // Check first tag creation events
        let tag1_creation_requested = events.get(0);
        assert!(tag1_creation_requested.is_some());
        let tag1_creation_requested = tag1_creation_requested.unwrap();
        let tag1_id = match tag1_creation_requested {
            OperationEvent::TagCreationRequested(data) => {
                assert_tag_creation_requested_event(data.clone(), command.clone(), 0)
            }
            _ => {
                panic!("Unexpected event type");
            }
        };

        // Check second tag creation events
        let tag2_creation_requested = events.get(1);
        assert!(tag2_creation_requested.is_some());
        let tag2_creation_requested = tag2_creation_requested.unwrap();
        let tag2_id = match tag2_creation_requested {
            OperationEvent::TagCreationRequested(data) => {
                assert_tag_creation_requested_event(data.clone(), command.clone(), 1)
            }
            _ => {
                panic!("Unexpected event type");
            }
        };

        // Check operation created event
        let operation_created = events.get(2);
        assert!(operation_created.is_some());
        let operation_created = operation_created.unwrap();
        match operation_created {
            OperationEvent::OperationCreated(data) => {
                assert_operation_created_event(data.clone(), command.clone());

                assert_eq!(data.payload().category_id().value(), command.category_id().unwrap());

                assert_eq!(data.payload().tag_ids().len(), 2);
                assert!(data.payload().tag_ids().contains(&Id::new(tag1_id)) && data.payload().tag_ids().contains(&Id::new(tag2_id)));
            }
            _ => {
                panic!("Unexpected event type");
            }
        };
    }

    #[test]
    pub fn test_operation_creation_without_tags() {
        let command = create_operation_command_fixture(true, false, false);

        let result = Operation::handle_creation(command.clone());

        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 1);

        let operation_created = events.get(0);
        assert!(operation_created.is_some());
        let operation_created = operation_created.unwrap();
        match operation_created {
            OperationEvent::OperationCreated(data) => {
                assert_operation_created_event(data.clone(), command.clone());

                assert_eq!(data.payload().category_id().to_string().len(), 36);
                assert_eq!(data.payload().tag_ids().len(), 0);
            }
            _ => {
                panic!("Unexpected event type");
            }
        };
    }

    #[test]
    pub fn test_operation_creation_with_incorrect_amount() {
        let user_id = Id::generate();
        let category_id = Id::generate();

        let command = CreateOperationCommand::new(
            String::from("Income"),
            user_id,
            Some(category_id),
            String::from("Food"),
            300.0,
            String::from("USD"),
            100.0,
            2.0,
            String::from("Grocery Shopping"),
            vec![],
        );

        let result = Operation::handle_creation(command.clone());

        assert!(result.is_err());

        let error = result.unwrap_err();
        match error {
            DomainError::InvalidAmount(message) => {
                assert_eq!(message, "Amount 300 is not equal to currency amount 100 by rate 2");
            }
            _ => {
                panic!("Unexpected error type");
            }
        };
    }

    pub fn create_operation_command_fixture(has_category_id: bool, has_tags: bool, has_new_tags: bool) -> CreateOperationCommand {
        let user_id = Id::generate();

        let category_id = if has_category_id {
            Some(Id::generate())
        } else {
            None
        };

        let tags = if has_tags {
            let (tag1_id, tag2_id) = if has_new_tags {
                (None, None)
            } else {
                (Some(Id::generate()), Some(Id::generate()))
            };

            vec![
                TagData::new(tag1_id, String::from("groceries")),
                TagData::new(tag2_id, String::from("essentials")),
            ]
        } else {
            vec![]
        };

        CreateOperationCommand::new(
            String::from("Income"),
            user_id,
            category_id,
            String::from("Food"),
            100.0,
            String::from("USD"),
            100.0,
            1.0,
            String::from("Grocery Shopping"),
            tags,
        )
    }

    fn assert_operation_created_event(data: OperationCreated, command: CreateOperationCommand) {
        assert_eq!(data.id().to_string().len(), 36);
        assert_eq!(data.payload().id().to_string().len(), 36);
        assert_eq!(data.payload().user_id().value(), *command.user_id());
        assert_eq!(data.payload().kind().to_str(), command.kind());
        assert_eq!(data.payload().amount().value(), command.amount());
        assert_eq!(data.payload().amount_currency().value(), command.currency_amount());
        assert_eq!(data.payload().currency().to_str(), command.currency());
        assert_eq!(data.payload().rate().value(), command.rate());
        assert_eq!(data.payload().label(), command.label());
    }

    fn assert_category_creation_requested_event(data: CategoryCreationRequested, command: CreateOperationCommand) {
        assert_eq!(data.payload().operation_id().to_string().len(), 36);
        assert_eq!(data.payload().user_id().value(), *command.user_id());
        assert_eq!(data.payload().category_id().to_string().len(), 36);
        assert_eq!(data.payload().category_name(), command.category_name());
    }

    fn assert_tag_creation_requested_event(data: TagCreationRequested, command: CreateOperationCommand, command_tag_index: usize) -> Uuid {
        assert_eq!(data.payload().operation_id().to_string().len(), 36);
        assert_eq!(data.payload().user_id().value(), *command.user_id());
        assert_eq!(data.payload().tag_id().to_string().len(), 36);
        assert_eq!(data.payload().tag_name(), command.tags()[command_tag_index].name());

        data.payload().tag_id().value().clone()
    }
}

