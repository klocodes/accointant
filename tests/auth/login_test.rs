use actix_web::test;
use actix_web::web::Data;
use actix_web::{App};
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
use metan::test_utils::environment::Environment;
use metan::http::handlers::auth::login::login;
use metan::services::tokenizer::Tokenizer;

#[actix_rt::test]
async fn test_login_successful() {
    let environment = Environment::new();
    let (service_container, event_bus, _) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(login)
    ).await;

    // Prepare data
    let id = Uuid::new_v4();
    let email = format!("{}@test.com", id);
    let password = bcrypt::hash("password", 10).expect("Failed to hash password");

    let tokenizer = service_container.tokenizer();
    let token = tokenizer.generate().expect("Failed to generate token");

    let expires_at = chrono::Utc::now() + chrono::Duration::hours(2);
    let confirmed_at = chrono::Utc::now();

    let pool = PgPool::connect(Environment::db_url().as_str()).await.expect("Failed to connect to database");
    let _ = sqlx::query("INSERT INTO users (id, email, password, confirmation_token, confirmation_token_expires_at, confirmed_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&id)
        .bind(&email)
        .bind(&password)
        .bind(&token)
        .bind(&expires_at)
        .bind(&confirmed_at)
        .execute(&pool)
        .await
        .expect("Failed to insert user");

    // Successful confirmation
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&serde_json::json!({
            "email": email,
            "password": "password"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}