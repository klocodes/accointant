use crate::support::id::Id;
use crate::features::tags::domain::events::tag_created::TagCreated;
use crate::features::tags::domain::events::tag_event::TagEvent;
use crate::features::tags::application::commands::create_tag::command::CreateTagCommand;
use crate::features::tags::domain::error::DomainError;

pub struct Tag {
    id: Id,
    user_id: Id,
    name: String,
}

impl Tag {
    pub fn handle_creation(command: CreateTagCommand) -> Result<TagEvent, DomainError> {
        let tag = Self {
            id: Id::new(Id::generate()),
            user_id: Id::new(command.user_id().clone()),
            name: command.tag_name().to_string(),
        };

        let event = TagEvent::TagCreated(
            TagCreated::new(
                Id::new(Id::generate()),
                tag.id().clone(),
                tag.user_id().clone(),
                tag.name().to_string(),
            )
        );

        Ok(event)
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation_successful() {
        let user_id = Id::generate();
        let tag_name = "tag_name".to_string();
        let command = CreateTagCommand::new(user_id, tag_name.clone());
        let event = Tag::handle_creation(command).unwrap();

        match event {
            TagEvent::TagCreated(event) => {
                assert_eq!(event.id().value().to_string().len(), 36);
                assert_eq!(event.payload().id().value().to_string().len(), 36);
                assert_eq!(event.payload().user_id().value().to_string().len(), 36);
                assert_eq!(event.payload().name(), tag_name);
            }
        }
    }
}