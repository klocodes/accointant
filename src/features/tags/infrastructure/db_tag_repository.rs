use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{query, Row};
use tokio::sync::Mutex;
use crate::db::manager::DbManager;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;
use crate::features::tags::domain::tag_repository::TagRepository;
use crate::features::tags::domain::events::tag_created::TagCreated;
use crate::services::serializer::Serializer;

#[derive(Clone)]
pub struct DbTagRepository {
    db_manager: Arc<Mutex<DbManager>>,
    serializer: Serializer,
}

impl DbTagRepository {
    pub fn new(db_manager: Arc<Mutex<DbManager>>, serializer: Serializer) -> Self {
        Self {
            db_manager,
            serializer,
        }
    }
}

#[async_trait]
impl TagRepository for DbTagRepository {
    async fn exists(&self, tag_created_name: &str, tag_deleted_name: &str, name: &str) -> Result<bool, Error> {
        let guard = self.db_manager.lock().await;
        let pool = guard.pool()?;

        let q = "
            SELECT EXISTS (
                SELECT 1
                FROM tag_events
                WHERE payload->>'name' = $1
                    AND name = $2
            AND NOT EXISTS (
                SELECT 1
                FROM tag_events
                WHERE payload->>'name' = $1
                    AND name = $3
            )
            ) AS exists
        ";

        let query = query(q)
            .bind(name)
            .bind(tag_created_name)
            .bind(tag_deleted_name);

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

    async fn persist_tag_created_event(&self, tag_created: &TagCreated) -> Result<(), Error> {
        let q = "INSERT INTO tag_events (id, name, payload) VALUES ($1, $2, $3)";

        let payload = serde_json::to_value(
            &tag_created.payload()
        ).map_err(|e|
            Error::Server(
                InternalServerError {
                    context: Some(e.to_string().into())
                }
            )
        )?;

        let query = query(q)
            .bind(tag_created.id().value())
            .bind(tag_created.name())
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