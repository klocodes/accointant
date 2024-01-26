use async_trait::async_trait;
use uuid::Uuid;
use crate::db::db_manager::TransactionManager;
use crate::errors::Error;
use crate::features::auth::domain::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;

    async fn find_by_email(&self, email: String) -> Result<Option<User>, Error>;

    async fn email_exists(&self, email: &str) -> Result<bool, Error>;

    async fn create(&self, transaction_manager: &mut TransactionManager, user: &User) -> Result<(), Error>;

    async fn confirm_email(&self, user: User) -> Result<(), Error>;

    async fn update_confirmation_token(&self, transaction_manager: &mut TransactionManager, user: User) -> Result<(), Error>;
}