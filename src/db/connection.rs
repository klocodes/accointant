use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::config::db::PostgresConfig;

pub type DbPool = Pool<Postgres>;

pub async fn connect(cfg: &PostgresConfig) -> DbPool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.user(),
        cfg.password(),
        cfg.host(),
        cfg.port(),
        cfg.database_name()
    );


    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Could not connect to the database")
}