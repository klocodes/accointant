use actix::Actor;
use crate::config::actor::ConfigActor;

mod config;
mod errors;
mod features;
mod http;
mod log;

#[actix_web::main]
async fn main() {
    let config = config::Config::new();
    let log_config = config.log();

    println!("log_level: {}", log_config.level());
    println!("log_level: {}", log_config.rotation());

    let server_config = config.server();
    let config_actor = ConfigActor::new(config.clone()).start();

    let _guard = log::logger::init(log_config).await.unwrap();

    http::server::run(server_config, config_actor).await.expect("Failed to start server");

    std::mem::forget(_guard);
}
