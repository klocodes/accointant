use tracing_appender::{non_blocking};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::{Subscriber};
use tracing_subscriber::fmt::time::ChronoUtc;

use crate::config::structs::log::LogConfig;

const LOG_PATH: &str = "log";

pub async fn init(config: LogConfig) -> Result<WorkerGuard, Box<dyn std::error::Error>>{
    let path: String = calculate_log_path();
    let rotation_period: &str = config.rotation();

    let rotation: Rotation;
    match rotation_period {
        "minutely" => rotation = Rotation::MINUTELY,
        "hourly" => rotation = Rotation::HOURLY,
        "daily" => rotation = Rotation::DAILY,
        "never" => rotation = Rotation::NEVER,
        _ => rotation = Rotation::NEVER,
    }
    let file_appender = RollingFileAppender::new(rotation, path.clone(), format!("log_{}", config.level()));

    let (non_blocking, _guard) = non_blocking(file_appender);

    let builder = Subscriber::builder()
        .with_env_filter(config.level())
        .with_writer(non_blocking)
        .with_timer(ChronoUtc::rfc_3339())
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    let subscriber = builder.finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    Ok(_guard)
}

fn calculate_log_path() -> String {
    let project_root = std::env::var("PROJECT_ROOT")
        .expect("The PROJECT_ROOT environment variable is not set");

    format!("{}/{}", project_root, LOG_PATH)
}
