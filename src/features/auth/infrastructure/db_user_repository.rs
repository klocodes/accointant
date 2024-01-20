use async_trait::async_trait;
use sqlx::{query, query_as, Row};
use crate::bootstrap::app_context::AppContext;
use crate::db::db_manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::infrastructure::user_schema::UserSchema;
use crate::service::data_mapper::DataMapper;

pub struct DbUserRepository {
    app_context: AppContext,
}

impl DbUserRepository {
    pub fn new(app_context: AppContext) -> Self {
        Self { app_context }
    }
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let q = "SELECT * FROM users WHERE email = $1";

        let res_query = query_as::<_, UserSchema>(q).bind(email);

        let pool = self.app_context.get_db_manager().pool().await?;

        let user_schema_option = res_query.fetch_optional(&pool).await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to fetch email: {}", e.to_string()).into())
            }))?;

        match user_schema_option {
            Some(user_schema) => {
                let user = UserSchema::decode(&user_schema);
                Ok(Some(user?))
            }
            None => Ok(None),
        }
    }

    async fn email_exists(&self, email: &str) -> Result<bool, Error> {
        let q = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)";

        let res_query = query(q).bind(email);

        let pool = self.app_context.get_db_manager().pool().await?;

        let row = res_query.fetch_one(&pool).await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to fetch email to check exists: {}", e.to_string()).into())
            }))?;

        row.try_get::<bool, _>(0)
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to check email exists: {}", e.to_string()).into())
            }))
    }


    async fn create(&self, user: &User) -> Result<(), Error> {
        let user_schema = UserSchema::encode(user)?;

        let q = "INSERT INTO users (id, name, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)";

        let res_query = query(q)
            .bind(user_schema.id())
            .bind(user_schema.name())
            .bind(user_schema.email())
            .bind(user_schema.password())
            .bind(user_schema.created_at())
            .bind(user_schema.updated_at());

        let pool = self.app_context.get_db_manager().pool().await?;

        res_query.execute(&pool).await
            .map_err(|e| Error::Server(ServerErrors::InternalServerError {
                context: Some(format!("Failed to create user: {}", e.to_string()).into())
            }))?;

        Ok(())
    }
}