use std::sync::Arc;
use async_trait::async_trait;
use sqlx::query;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::features::account::domain::account_repository::AccountRepository;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::error::AccountError;
use crate::features::account::infrastructure::error::InfrastructureError;

pub struct DbAccountRepository {
    db_manager: Arc<Mutex<DbManager>>,
}

impl DbAccountRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>) -> Self {
        Self {
            db_manager,
        }
    }
}

#[async_trait]
impl AccountRepository for DbAccountRepository {
    async fn persist_account_created_event(&self, event: AccountCreated) -> Result<(), AccountError> {
        let db_manager = self.db_manager.lock().await;
        let pool = db_manager.pool()
            .map_err(|e|
                AccountError::Infrastructure(
                    InfrastructureError::Repository(e.to_string())
                )
            )?;

        let payload = serde_json::to_value(&event)
            .map_err(|e|
                AccountError::Infrastructure(
                    InfrastructureError::Repository(e.to_string())
                )
            )?;

        let _ = query(
            r#"
                INSERT INTO account_created (id, name, payload)
                VALUES ($1, $2, $3)
            "#,
        )
            .bind(event.id().value())
            .bind(event.name().to_string())
            .bind(payload)
            .execute(&pool)
            .await
            .map_err(|e|
                AccountError::Infrastructure(
                    InfrastructureError::Repository(e.to_string())
                )
            )?;

        Ok(())
    }
}