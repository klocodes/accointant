use std::vec;
use async_trait::async_trait;
use crate::errors::Error;
use crate::features::auth::domain::user::User;

pub trait UserRepository {
    fn email_exists(&self, email: &str) -> Result<bool, Error>;
    fn create(&self, user: &User) -> Result<(), Error>;
}