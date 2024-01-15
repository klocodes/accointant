use std::time::Duration;
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
        cfg.name()
    );

println!("database_url: {}", database_url);
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(30))
        .max_connections(100)
        .connect(database_url.as_str())
        .await
        .expect("Could not connect to the database")
}