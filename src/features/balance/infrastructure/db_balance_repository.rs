use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::features::balance::domain::balance_repository::BalanceRepository;
use crate::features::balance::domain::events::balance_changed::BalanceChanged;
use crate::features::balance::error::BalanceError;
use crate::features::balance::infrastructure::error::InfrastructureError;

pub struct DbBalanceRepository {
    db_manager: Arc<Mutex<DbManager>>,
}

impl DbBalanceRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>) -> Self {
        Self { db_manager }
    }
}

#[async_trait]
impl BalanceRepository for DbBalanceRepository {
    async fn persist_balance_changed_event(&self, balance_changed: &BalanceChanged) -> Result<(), BalanceError> {
        let pool = self.db_manager.lock()
            .await
            .pool()
            .map_err(|e|
                BalanceError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get pool: {}", e.to_string())
                    )
                )
            )?;

        let payload = serde_json::to_value(balance_changed.payload())
            .map_err(|e|
                BalanceError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to serialize balance event payload: {}", e.to_string())
                    )
                )
            )?;

        let _ = sqlx::query(
            r#"
            INSERT INTO balance_changeds (id, name, payload)
            VALUES ($1, $2, $3)
            "#
        ).bind(balance_changed.id().value())
            .bind(balance_changed.name().to_string())
            .bind(payload)
            .execute(&pool)
            .await
            .map_err(|e|
                BalanceError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to persist balance event: {}", e.to_string())
                    )
                )
            )?;

        Ok(())
    }
}