use actix::Actor;
use crate::config::actor::ConfigActor;
use crate::db::connection::connect;
use crate::db::data_mapper::DataMapper;
use crate::errors::Error;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;
use crate::features::auth::domain::user::User;
use crate::features::auth::infrastructure::mapping::user_mapper::UserMapper;
use crate::features::auth::infrastructure::mapping::user_schema::UserSchema;

mod config;
mod errors;
mod features;
mod http;
mod log;
mod db;


#[actix_web::main]
async fn main() {
    let user = User::new(
        Email::new("test@tesdt.com").unwrap(),
        Password::new("12345678987654321").unwrap(),
        Password::new("12345678987654321").unwrap(),
    ).unwrap();

    let user_schema: UserSchema = UserMapper::encode(&user).unwrap();
    let user: Result<User, Error> = UserMapper::decode(&user_schema);

    println!("Schema: {:?}", user_schema);
    println!("Entity: {:?}", user.unwrap());

    let config = config::Config::new();
    let log_config = config.log();
    let db_config = config.db().pg();
    let server_config = config.server();

    let config_actor = ConfigActor::new(config.clone()).start();

    let _guard = log::logger::init(log_config).await.unwrap();

    let pool = connect(db_config).await;

    http::server::run(server_config, config_actor, pool).await.expect("Failed to start server");

    std::mem::forget(_guard);
}
