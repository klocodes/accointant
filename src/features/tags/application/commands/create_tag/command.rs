use uuid::Uuid;
use crate::support::command_bus::Command;

const NAME : &str = "create_tag";

pub struct CreateTagCommand {
    user_id: Uuid,
    tag_name: String,
}

impl CreateTagCommand {
    pub fn new(user_id: Uuid, tag_name: String) -> Self {
        Self {
            user_id,
            tag_name,
        }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }
}

impl Command for CreateTagCommand {
    fn name() -> &'static str {
        NAME
    }
}

