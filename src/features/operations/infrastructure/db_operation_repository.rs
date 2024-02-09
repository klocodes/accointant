use std::sync::Arc;
use async_trait::async_trait;
use sqlx::query;
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::operations::domain::events::operation_created::OperationCreated;
use crate::features::operations::domain::operation_repository::OperationRepository;
use crate::features::operations::infrastructure::schema::operation_created_schema::OperationCreatedEventSchema;
use crate::services::serializer::Serializer;
use crate::support::data_mapper::DataMapper;

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
    async fn persist_operation_created_event(&self, event_data: OperationCreated) -> Result<(), Error> {
        let fields_str = "id, operation_id, category_id, user_id, kind, amount, amount_currency, currency, rate, label, created_at";
        let args_str = "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11";
        let q = format!("INSERT INTO operations_created ({}) VALUES ({})", fields_str, args_str);

        let operation_schema = OperationCreatedEventSchema::encode(self.serializer.clone(), &event_data)?;

        let res_query = query(&q)
            .bind(operation_schema.id())
            .bind(operation_schema.operation_id())
            .bind(operation_schema.user_id())
            .bind(operation_schema.kind())
            .bind(operation_schema.category_id())
            .bind(operation_schema.amount())
            .bind(operation_schema.amount_currency())
            .bind(operation_schema.currency())
            .bind(operation_schema.rate())
            .bind(operation_schema.label())
            .bind(operation_schema.created_at());

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
                        format!("Failed to execute query to register user: {}", e.to_string()).into()
                    )
                }
            )
        })?;

        Ok(())
    }
}