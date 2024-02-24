use std::sync::Arc;
use async_trait::async_trait;
use sqlx::query;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::services::serializer::Serializer;

pub struct DbOperationRepository {
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
}

impl DbOperationRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>, serializer: Serializer) -> Self {
        Self {
            db_manager,
            serializer,
        }
    }
}

#[async_trait]
impl OperationRepository for DbOperationRepository {
    async fn persist_operation_created_event(&self, operation_created: OperationCreated) -> Result<(), Error> {
        let q = "INSERT INTO operation_events (id, name, payload) VALUES ($1, $2, $3)";

        let payload = serde_json::to_value(operation_created.payload()).map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to serialize operation created payload: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        let res_query = query(&q)
            .bind(operation_created.id().value())
            .bind(operation_created.name())
            .bind(&payload);

        let mut guard = self.db_manager.lock().await;
        guard.begin().await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to begin transaction: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        let tx = guard.transaction().await?;

        res_query.execute(&mut **tx).await.map_err(|e| {
            Error::Server(
                InternalServerError {
                    context: Some(
                        format!("Failed to execute query to create operation: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }
}