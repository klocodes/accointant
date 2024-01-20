/*use crate::db::query_builder::QueryBuilder;
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;

pub struct DbUserRepository<'a> {
    query_builder: QueryBuilder<'a>,
}

impl DbUserRepository<'_> {
    pub fn new(query_builder: QueryBuilder) -> Self {
        Self { query_builder }
    }
}

impl UserRepository for DbUserRepository<'_> {
    fn email_exists(&self, email: &str) -> Result<bool, Error> {


       Ok(true)
    }


    fn create(&self, user: &User) -> Result<(), Error> {
        Ok(())
    }
}*/