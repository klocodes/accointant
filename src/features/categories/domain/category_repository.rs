use async_trait::async_trait;
use mockall::automock;
use crate::features::categories::domain::events::category_created::CategoryCreated;
use crate::features::categories::error::CategoryError;

#[async_trait]
#[automock]
pub trait CategoryRepository {
    async fn exists(&self, category_created_name: &str, category_deleted_name: &str, name: &str) -> Result<bool, CategoryError>;

    async fn persist_category_created_event(&self, category: &CategoryCreated) -> Result<(), CategoryError>;
}