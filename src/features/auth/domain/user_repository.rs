use async_trait::async_trait;
use uuid::Uuid;
use crate::features::auth::domain::user::User;
use crate::features::auth::error::AuthError;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError>;

    async fn find_by_email(&self, email: String) -> Result<Option<User>, AuthError>;

    async fn email_exists(&self, email: &str) -> Result<bool, AuthError>;

    async fn create(&self, user: &User) -> Result<(), AuthError>;

    async fn confirm_email(&self, user: User) -> Result<(), AuthError>;

    async fn update_confirmation_token(&self, user: User) -> Result<(), AuthError>;
}