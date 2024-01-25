use async_trait::async_trait;
use sqlx::{query, query_as, Row};
use uuid::Uuid;

use crate::bootstrap::app_context::{AppContext, TransactionManager};
use crate::db::db_manager::DbManager;
use crate::db::db_transaction::DbTransaction;
use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::infrastructure::user_schema::UserSchema;
use crate::services::data_mapper::DataMapper;

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
    async fn find_by_email(&self, email: String) -> Result<Option<User>, Error> {
        let q = "SELECT * FROM users WHERE email = $1";

        let res_query = query_as::<_, UserSchema>(q).bind(email);

        let pool = self.app_context.get_db_manager().pool().await?;

        let user_schema_option = res_query.fetch_optional(&pool).await
            .map_err(|e| Error::Server(InternalServerError {
                context: Some(format!("Failed to fetch user by email: {}", e.to_string()).into())
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
            .map_err(|e| Error::Server(InternalServerError {
                context: Some(format!("Failed to fetch email to check exists: {}", e.to_string()).into())
            }))?;

        row.try_get::<bool, _>(0)
            .map_err(|e| Error::Server(InternalServerError {
                context: Some(format!("Failed to check email exists: {}", e.to_string()).into())
            }))
    }


    async fn create(&self, transaction_manager: &mut TransactionManager, user: &User) -> Result<(), Error> {
        let user_schema = UserSchema::encode(user)?;

        let q = "INSERT INTO users (id, email, password, confirmation_token, confirmation_token_expires_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)";
        let res_query = query(q);

        let res_query = res_query.bind(user_schema.id())
            .bind(user_schema.email())
            .bind(user_schema.password())
            .bind(user_schema.confirmation_token())
            .bind(user_schema.confirmation_token_expires_at())
            .bind(user_schema.created_at())
            .bind(user_schema.updated_at());

        let pool = self.app_context.get_db_manager().pool().await?;

        transaction_manager.begin(pool).await?;

        let tx = transaction_manager.get().await?;

        res_query.execute(&mut **tx).await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to execute query to register user: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }

    async fn confirm_email(&self, id: Uuid) -> Result<(), Error> {
        let confirmed_at = chrono::Utc::now();

        let q = "UPDATE users SET confirmed_at = $1, updated_at = $2 WHERE id = $3";

        let mut res_query = query(q);

        res_query = res_query.bind(confirmed_at)
            .bind(confirmed_at)
            .bind(id);

        let pool = self.app_context.get_db_manager().pool().await?;

        res_query.execute(&pool).await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to execute query to confirm email: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }
}