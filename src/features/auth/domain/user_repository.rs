use async_trait::async_trait;
use crate::errors::Error;
use crate::features::auth::domain::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn email_exists(&self, email: &str) -> Result<bool, Error>;
    async fn create(&self, user: &User) -> Result<(), Error>;
}