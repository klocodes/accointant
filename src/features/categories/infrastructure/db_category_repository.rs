use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{query, Row};
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::categories::domain::category_repository::CategoryRepository;
use crate::features::categories::domain::events::category_created::CategoryCreated;
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
    async fn exists(&self, category_created_name: &str, category_deleted_name: &str, name: &str) -> Result<bool, Error> {
        let guard = self.db_manager.lock().await;
        let pool = guard.pool()?;

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
                Error::Server(
                    InternalServerError {
                        context: Some(e.to_string().into())
                    }
                )
            )?;

        let exists = row.try_get::<bool, _>(0)
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(e.to_string().into())
                    }
                )
            )?;

        Ok(exists)
    }

    async fn persist_category_created_event(&self, category_created: &CategoryCreated) -> Result<(), Error> {
        let q = "INSERT INTO category_events (id, name, payload) VALUES ($1, $2, $3)";

        let payload = serde_json::to_value(
            &category_created.payload()
        ).map_err(|e|
            Error::Server(
                InternalServerError {
                    context: Some(e.to_string().into())
                }
            )
        )?;

        let query = query(q)
            .bind(category_created.id().value())
            .bind(category_created.name())
            .bind(payload);

        let guard = self.db_manager.lock().await;

        query.execute(&guard.pool()?)
            .await
            .map_err(|e|
                Error::Server(
                    InternalServerError {
                        context: Some(e.to_string().into())
                    }
                )
            )?;

        Ok(())
    }
}