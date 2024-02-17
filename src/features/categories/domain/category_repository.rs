use async_trait::async_trait;
use crate::errors::Error;
use crate::features::categories::domain::events::category_created::CategoryCreated;

#[async_trait]
pub trait CategoryRepository {
    async fn exists(&self, category_created_name: &str, category_deleted_name: &str, name: &str) -> Result<bool, Error>;

    async fn persist_category_created_event(&self, category: &CategoryCreated) -> Result<(), Error>;
}