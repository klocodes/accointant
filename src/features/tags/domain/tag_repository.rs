use async_trait::async_trait;
use crate::features::tags::domain::events::tag_created::TagCreated;
use crate::features::tags::error::TagError;
use crate::features::tags::infrastructure::error::InfrastructureError;

#[async_trait]
pub trait TagRepository {
    async fn exists(&self, tag_created_event_name: &str, tag_deleted_event_name: &str, tag_name: &str) -> Result<bool, TagError>;

    async fn persist_tag_created_event(&self, tag: &TagCreated) -> Result<(), TagError>;
}

pub struct MockTagRepository {
    exists_method_has_error: bool,
    persist_tag_created_event_method_has_error: bool,
}

impl MockTagRepository {
    pub fn new(exists_method_has_error: bool, persist_tag_created_event_method_has_error: bool) -> Self {
        Self {
            exists_method_has_error,
            persist_tag_created_event_method_has_error,
        }
    }
}

#[async_trait]
impl TagRepository for MockTagRepository {
    async fn exists(&self, _tag_created_event_name: &str, _tag_deleted_event_name: &str, _tag_name: &str) -> Result<bool, TagError> {
        if self.exists_method_has_error {
            return Err(
                TagError::Infrastructure(
                    InfrastructureError::Repository("Tag already exists".into())
                )
            );
        }

        Ok(false)
    }

    async fn persist_tag_created_event(&self, _tag: &TagCreated) -> Result<(), TagError> {
        if self.persist_tag_created_event_method_has_error {
            return Err(
                TagError::Infrastructure(
                    InfrastructureError::Repository("Mock repository error".into())
                )
            );
        }

        Ok(())
    }
}