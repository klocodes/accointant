use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{query, query_as};
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::features::account::domain::account_repository::AccountRepository;
use crate::features::account::domain::events::account_created::AccountCreated;
use crate::features::account::domain::events::account_event::AccountEvent;
use crate::features::account::domain::events::account_operation_applied::AccountOperationApplied;
use crate::features::account::error::AccountError;
use crate::features::account::infrastructure::account_event_schema::AccountEventSchema;
use crate::features::account::infrastructure::error::InfrastructureError;
use crate::services::serializer::Serializer;
use crate::support::data_mapper::DataMapper;

pub struct DbAccountRepository {
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
}

impl DbAccountRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>, serializer: Serializer) -> Self {
        Self {
            db_manager,
            serializer,
        }
    }
}

#[async_trait]
impl AccountRepository for DbAccountRepository {
    async fn find_events_by_id(&self, account_id: uuid::Uuid) -> Result<Vec<AccountEvent>, AccountError> {
        let db_manager = self.db_manager.lock().await;
        let pool = db_manager.pool()
            .map_err(|e|
                AccountError::Infrastructure(
                    InfrastructureError::Repository(e.to_string())
                )
            )?;

        let mut account_event_schemas = query_as::<_, AccountEventSchema>(
            r#"
                SELECT * FROM account_events WHERE payload->>'id' = $1
            "#
        ).bind(account_id)
            .fetch_all(&pool)
            .await
            .map_err(|e|
                AccountError::Infrastructure(
                    InfrastructureError::Repository(e.to_string())
                )
            )?;

        account_event_schemas
            .iter_mut()
            .map(|mut schema| {
                schema.decode_payload()
                    .map_err(|e|
                        AccountError::Infrastructure(
                            InfrastructureError::Repository(e.to_string())
                        )
                    )?;

                AccountEventSchema::decode(self.serializer.clone(), schema)
                    .map_err(|e|
                        AccountError::Infrastructure(
                            InfrastructureError::Repository(e.to_string())
                        )
                    )
            })
            .collect::<Result<Vec<AccountEvent>, AccountError>>()
    }

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
                INSERT INTO account_events (id, name, payload)
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

    async fn persist_account_operation_applied_event(&self, _event: AccountOperationApplied) -> Result<(), AccountError> {
        unimplemented!()
    }
}