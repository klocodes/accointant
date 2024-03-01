use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{query, Row};
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::features::categories::domain::category_repository::CategoryRepository;
use crate::features::categories::domain::events::category_created::CategoryCreated;
use crate::features::categories::error::CategoryError;
use crate::features::categories::infrastructure::error::InfrastructureError;
use crate::services::serializer::Serializer;

#[derive(Clone)]
pub struct DbCategoryRepository {
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
}

impl DbCategoryRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>, serializer: Serializer) -> Self {
        Self {
            db_manager,
            serializer,
        }
    }
}

#[async_trait]
impl CategoryRepository for DbCategoryRepository {
    async fn exists(&self, category_created_name: &str, category_deleted_name: &str, name: &str) -> Result<bool, CategoryError> {
        let guard = self.db_manager.lock().await;
        let pool = guard.pool()
            .map_err(|e|
                CategoryError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get pool: {}", e.to_string())
                    )
                )
            )?;

        let q = "
            SELECT EXISTS (
                SELECT 1
                FROM category_events
                WHERE payload->>'name' = $1
                    AND name = $2
            AND NOT EXISTS (
                SELECT 1
                FROM category_events
                WHERE payload->>'name' = $1
                    AND name = $3
            )
            ) AS exists
        ";

        let query = query(q)
            .bind(name)
            .bind(category_created_name)
            .bind(category_deleted_name);

        let row = query.fetch_one(&pool).await
            .map_err(|e|
                CategoryError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to execute persist category creation event query: {}", e.to_string())
                    )
                )
            )?;

        let exists = row.try_get::<bool, _>(0)
            .map_err(|e|
                CategoryError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get exists value: {}", e.to_string())
                    )
                )
            )?;

        Ok(exists)
    }

    async fn persist_category_created_event(&self, category_created: &CategoryCreated) -> Result<(), CategoryError> {
        let q = "INSERT INTO category_events (id, name, payload) VALUES ($1, $2, $3)";

        let payload = serde_json::to_value(
            &category_created.payload()
        ).map_err(|e|
            CategoryError::Infrastructure(
                InfrastructureError::Repository(
                    format!("Failed to serialize category event payload: {}", e.to_string())
                )
            )
        )?;

        let query = query(q)
            .bind(category_created.id().value())
            .bind(category_created.name())
            .bind(payload);

        let guard = self.db_manager.lock().await;
        let pool = guard.pool()
            .map_err(|e|
                CategoryError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to get pool: {}", e.to_string())
                    )
                )
            )?;

        query.execute(&pool)
            .await
            .map_err(|e|
                CategoryError::Infrastructure(
                    InfrastructureError::Repository(
                        format!("Failed to persist category event: {}", e.to_string())
                    )
                )
            )?;

        Ok(())
    }
}