use async_trait::async_trait;
use crate::errors::client::ClientErrors::BadRequest;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::tags::domain::events::tag_created::TagCreated;

#[async_trait]
pub trait TagRepository {
    async fn exists(&self, tag_created_event_name: &str, tag_deleted_event_name: &str, tag_name: &str) -> Result<bool, Error>;

    async fn persist_tag_created_event(&self, tag: &TagCreated) -> Result<(), Error>;
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
    async fn exists(&self, _tag_created_event_name: &str, _tag_deleted_event_name: &str, _tag_name: &str) -> Result<bool, Error> {
        if self.exists_method_has_error {
            return Err(Error::Client(
                BadRequest {
                    message: Some("Error".into())
                }
            ));
        }

        Ok(false)
    }

    async fn persist_tag_created_event(&self, _tag: &TagCreated) -> Result<(), Error> {
        if self.persist_tag_created_event_method_has_error {
            return Err(Error::Server(
                InternalServerError {
                    context: Some("Error".into())
                }
            ));
        }

        Ok(())
    }
}