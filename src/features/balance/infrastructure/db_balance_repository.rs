use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::balance::domain::balance_repository::BalanceRepository;
use crate::features::balance::domain::events::balance_changed::BalanceChanged;

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
    async fn persist_balance_changed_event(&self, balance_changed: &BalanceChanged) -> Result<(), Error> {

        let pool = self.db_manager.lock().await.pool()?;
        let payload = serde_json::to_value(balance_changed.payload())
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to serialize balance event payload: {}", e.to_string()).into()
                        )
                    }
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
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to persist balance event: {}", e.to_string()).into()
                        )
                    }
                )
            )?;

        Ok(())
    }
}