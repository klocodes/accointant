mod environment;

use actix_web::test;
use actix_web::web::Data;
use actix_web::{App};
use sqlx::PgPool;
use uuid::Uuid;
use environment::Environment;
use metan::http::handlers::auth::request_confirmation_token::request;
use metan::services::tokenizer::Tokenizer;
#[actix_rt::test]
async fn test_request_confirmation_successful() {
    let environment = Environment::new();
    let (service_container, event_bus) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(request)
    ).await;

    // Prepare data
    let id = Uuid::new_v4();
    let email = "test@test.com";
    let password = bcrypt::hash("password", 10).expect("Failed to hash password");

    let tokenizer = service_container.tokenizer();
    let token = tokenizer.generate().expect("Failed to generate token");

    let expires_at = chrono::Utc::now() - chrono::Duration::hours(1);

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

    // Successful confirmation
    let req = test::TestRequest::post()
        .uri(format!("/request-confirmation-token/{}", id).as_str())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}