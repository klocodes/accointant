use actix::dev::Request;
use serde_json::json;
use actix_web::{test, App};
use actix_web::dev::Service;
use actix_web::web::Data;
use sqlx::PgPool;
use uuid::Uuid;
use metan::test_utils::environment::Environment;
use metan::services::tokenizer::Tokenizer;
use metan::http::handlers::auth::registration::register;

#[actix_rt::test]
async fn test_registration_and_confirmation() {
    let environment = Environment::new();
    let (service_container, event_bus) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(register)
    ).await;

    // Successful registration
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&json!({
            "email": format!("{}@test.com", Uuid::new_v4()),
            "password": "password",
            "password_confirmation": "password"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}

#[actix_rt::test]
async fn test_registration_email_exists() {
    let environment = Environment::new();
    let (service_container, event_bus) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(register)
    ).await;

    // Prepare data
    let id = Uuid::new_v4();
    let email = format!("{}@example.com", id);
    let password = bcrypt::hash("password", 10).expect("Failed to hash password");

    let tokenizer = service_container.tokenizer();
    let token = tokenizer.generate().expect("Failed to generate token");

    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

    let pool = PgPool::connect(Environment::db_url().as_str()).await.expect("Failed to connect to database");
    let _ = sqlx::query("INSERT INTO users (id, email, password, confirmation_token, confirmation_token_expires_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&email)
        .bind(&password)
        .bind(&token)
        .bind(&expires_at)
        .execute(&pool)
        .await
        .expect("Failed to insert user");

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&json!({
            "email": email,
            "password": "password",
            "password_confirmation": "password",
        }));

    let resp = test::call_service(&app, req.to_request()).await;

    assert_eq!(resp.status(), 400);
}
