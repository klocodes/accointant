use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{query, query_as, Row};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::db::manager::DbManager;
use crate::features::auth::domain::user::User;
use crate::features::auth::domain::user_repository::UserRepository;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::features::auth::infrastructure::user_schema::UserSchema;
use crate::services::serializer::Serializer;
use crate::support::data_mapper::DataMapper;

pub struct DbUserRepository {
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
}

impl DbUserRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>, serializer: Serializer) -> Self {
        Self {
            db_manager,
            serializer,
        }
    }
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AuthError> {
        let q = "SELECT * FROM users WHERE id = $1";

        let res_query = query_as::<_, UserSchema>(q).bind(user_id);

        let guard = self.db_manager.lock().await;
        let pool = guard.pool().map_err(|e|
            AuthError::Infrastructure(
                InfrastructureError::Repository(
                    format!("Failed to get pool: {}", e.to_string())
                )
            )
        )?;

        let user_schema_option = res_query.fetch_optional(&pool).await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to fetch user by id: {}", e.to_string())
                    )
                )
            )?;

        match user_schema_option {
            Some(user_schema) => {
                let user = UserSchema::decode(self.serializer.clone(), &user_schema)
                    .map_err(|e|
                        AuthError::Infrastructure(
                            InfrastructureError::Repository(
                                format!("Failed to map user: {}", e.to_string())
                            )
                        )
                    )?;

                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>, AuthError> {
        let q = "SELECT * FROM users WHERE email = $1";

        let res_query = query_as::<_, UserSchema>(q).bind(email);

        let guard = self.db_manager.lock().await;
        let pool = guard.pool().map_err(
            |e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get pool: {}", e.to_string())
                    )
                )
        )?;

        let user_schema_option = res_query.fetch_optional(&pool).await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to fetch user by email: {}", e.to_string())
                    )
                )
            )?;

        match user_schema_option {
            Some(user_schema) => {
                let user = UserSchema::decode(self.serializer.clone(), &user_schema)
                    .map_err(|e|
                        AuthError::Infrastructure(
                            InfrastructureError::Repository(
                                format!("Failed to map user: {}", e.to_string())
                            )
                        )
                    )?;

                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AuthError> {
        let q = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)";

        let res_query = query(q).bind(email);

        let guard = self.db_manager.lock().await;
        let pool = guard.pool()
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get pool: {}", e.to_string())
                    )
                )
            )?;

        let row = res_query.fetch_one(&pool).await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to fetch email to check exists: {}", e.to_string())
                    )
                )
            )?;

        row.try_get::<bool, _>(0)
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get email exists: {}", e.to_string())
                    )
                )
            )
    }


    async fn create(&self, user: &User) -> Result<(), AuthError> {
        let user_schema = UserSchema::encode(self.serializer.clone(), user).map_err(|e|
            AuthError::Infrastructure(
                InfrastructureError::Repository(
                    format!("Failed to map user: {}", e.to_string())
                )
            )
        )?;

        let q = "INSERT INTO users (id, email, password, confirmation_token, confirmation_token_expires_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)";
        let res_query = query(q);

        let res_query = res_query.bind(user_schema.id())
            .bind(user_schema.email())
            .bind(user_schema.password())
            .bind(user_schema.confirmation_token())
            .bind(user_schema.confirmation_token_expires_at())
            .bind(user_schema.created_at())
            .bind(user_schema.updated_at());

        let mut guard = self.db_manager.lock().await;
        guard.begin().await.map_err(|e|
            AuthError::Infrastructure(
                InfrastructureError::Transaction(
                    format!("Failed to begin transaction: {}", e.to_string())
                )
            )
        )?;

        let tx = guard.transaction()
            .await
            .map_err(
                |e|
                    AuthError::Infrastructure(
                        InfrastructureError::Transaction(
                            format!("Failed to begin transaction: {}", e.to_string())
                        )
                    )
            )?;

        res_query.execute(&mut **tx)
            .await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to execute query to register user: {}", e.to_string())
                    )
                )
            )?;

        Ok(())
    }

    async fn confirm_email(&self, user: User) -> Result<(), AuthError> {
        let q = "UPDATE users SET confirmed_at = $1, updated_at = $2 WHERE id = $3";

        let res_query = query(q)
            .bind(user.confirmed_at())
            .bind(user.updated_at())
            .bind(user.id());

        let guard = self.db_manager.lock().await;
        let pool = guard.pool().map_err(|e|
            AuthError::Infrastructure(
                InfrastructureError::Repository(
                    format!("Failed to get pool: {}", e.to_string())
                )
            )
        )?;

        res_query.execute(&pool)
            .await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to execute query to confirm email: {}", e.to_string())
                    )
                )
            )?;

        Ok(())
    }

    async fn update_confirmation_token(&self, user: User) -> Result<(), AuthError> {
        let q = "UPDATE users SET confirmation_token = $1, confirmation_token_expires_at = $2, updated_at = $3 WHERE id = $4";
        let res_query = query(q)
            .bind(user.confirmation_token().value())
            .bind(user.confirmation_token().expires_at())
            .bind(user.updated_at())
            .bind(user.id());

        let mut guard = self.db_manager.lock().await;
        guard.begin().await.map_err(|e|
            AuthError::Infrastructure(
                InfrastructureError::Transaction(
                    format!("Failed to begin transaction: {}", e.to_string())
                )
            )
        )?;

        let tx = guard.transaction()
            .await
            .map_err(
                |e|
                    AuthError::Infrastructure(
                        InfrastructureError::Transaction(
                            format!("Failed to get transaction: {}", e.to_string())
                        )
                    )
            )?;

        res_query.execute(&mut **tx)
            .await
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to execute query to register user: {}", e.to_string())
                    )
                )
            )?;

        Ok(())
    }
}