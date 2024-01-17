use async_trait::async_trait;
use crate::db::data_mapper::DataMapper;
use crate::db::manager::db_manager::DbManager;
use crate::errors::Error;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::infrastructure::user_schema::{UserSchema};

pub struct DbUserRepository {
    db_manager: DbManager,
}

impl DbUserRepository {
    pub fn new(db_manager: DbManager) -> Self {
        Self {db_manager}
    }
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let query = "SELECT * FROM users WHERE email = $1";
        let users: Vec<UserSchema> = self.db_manager.execute_query(query, &vec![email]).await?;

        let user = if let Some(schema) = users.first() {
            Some(UserSchema::decode(schema)?)
        } else {
            None
        };

        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<(), Error> {

        Ok(())
    }
}