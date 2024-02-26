use serde::{Deserialize, Serialize};
use crate::features::shared::id::Id;

pub const TAG_DELETED_NAME: &str = "tag_deleted";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagDeleted {
    id: Id,
    name: String,
}