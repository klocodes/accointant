use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc};
use metan::config::manager::ConfigManager;
use metan::di::service_container::ServiceContainer;
use metan::events::event_bus::EventBus;
use metan::events::event_bus_factory::EventBusFactory;

pub struct Environment {
    db_url: String,
}

impl Environment {
    pub fn new() -> Self {
        let _ = env_logger::builder().is_test(true).try_init();

        dotenv::from_filename(".env.test").ok();

        let db_url = Self::db_url();

        Self {
            db_url,
        }
    }

    // Load environment variables from .env file
    pub async fn setup(&self) -> (Arc<ServiceContainer>, Arc<Box<dyn EventBus>>) {
        self.wait_for_db_ready().await;
        self.migrate();

        // Start the application in a separate thread
        let config = ConfigManager::new();

        let service_container = ServiceContainer::new(config).await.expect("Failed to create service container");
        let service_container = Arc::new(service_container);

        let (event_bus, receiver) = EventBusFactory::create(service_container.clone()).await.expect("Failed to create event bus");

        let event_bus_clone = event_bus.clone();
        tokio::spawn(async move {
            event_bus_clone.start(receiver).await.expect("Failed to start event bus");
        });

        (service_container, event_bus)
    }

    pub fn db_url() -> String {
        format!("postgres://{}:{}@{}:{}/{}?sslmode=disable",
                env::var("DB_USER").unwrap_or("postgres".to_string()),
                env::var("DB_PASSWORD").unwrap_or("password".to_string()),
                env::var("DB_HOST").unwrap_or("localhost".to_string()),
                env::var("DB_PORT").unwrap_or("5432".to_string()),
                env::var("DB_NAME").unwrap_or("postgres".to_string())
        )
    }

    fn migrate(&self) {
        println!("Cleanup migrations database...");
        let _output = Command::new("just")
            .arg("test-migrate")
            .arg(&self.db_url)
            .arg("down -all")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to down migration");

        println!("Running migrations...");
        let _output = Command::new("just")
            .arg("test-migrate")
            .arg(&self.db_url)
            .arg("up")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run migration");
    }


    async fn wait_for_db_ready(&self) {
        let mut retries = 5;
        while retries > 0 {
            if sqlx::PgPool::connect(&self.db_url.as_str()).await.is_ok() {
                println!("Database is ready for connections.");
                return;
            } else {
                println!("Waiting for database to be ready...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                retries -= 1;
            }
        }
        panic!("Database did not become ready in time.");
    }
}
