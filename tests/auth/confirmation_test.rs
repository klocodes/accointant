use actix_web::test;
use actix_web::web::Data;
use actix_web::{App};
use sqlx::PgPool;
use uuid::Uuid;
use metan::test_utils::environment::Environment;
use metan::http::handlers::auth::confirm_registration::confirm;
use metan::services::tokenizer::Tokenizer;

#[actix_rt::test]
async fn test_confirm_registration_successful() {
    let environment = Environment::new();
    let (service_container, event_bus, _) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(confirm)
    ).await;

    // Prepare data
    let id = Uuid::new_v4();
    let email = format!("{}@test.com", id);
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

    // Successful confirmation
    let req = test::TestRequest::get()
        .uri(format!("/confirm?id={}&token={}", id, token).as_str())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}