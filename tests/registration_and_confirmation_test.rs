mod environment;

use actix::dev::Request;
use serde_json::json;
use actix_web::{test, App};
use actix_web::dev::Service;
use actix_web::web::Data;
use sqlx::PgPool;
use sqlx::Row;

use environment::Environment;
use metan::http::handlers::auth::confirm_registration::confirm;
use metan::http::handlers::auth::registration::{register, ResponseData};
use metan::http::handlers::auth::request_confirmation_token::request;

#[actix_rt::test]
async fn test_registration_and_confirmation() {
    let environment = Environment::new();
    let (service_container, event_bus) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container))
            .app_data(Data::new(event_bus))
            .service(register)
            .service(confirm)
            .service(request)
    ).await;

    // Successful registration
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&json!({
            "email": "test@example.com",
            "password": "password",
            "password_confirmation": "password"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body = test::read_body(resp).await;

    let response_data: ResponseData = serde_json::from_slice(&body).expect("Failed to deserialize response");

    // User already exists
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&json!({
            "email": "test@example.com",
            "password": "password",
            "password_confirmation": "password"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);

    // Confirm registration
    let pool = PgPool::connect(Environment::db_url().as_str()).await.expect("Failed to connect to database");

    let row = sqlx::query("SELECT confirmation_token FROM users WHERE id = $1")
        .bind(response_data.id)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch token");

    let token = row.try_get::<String, _>("confirmation_token").expect("Failed to get token");

    // Successful confirmation
    let req = test::TestRequest::get()
        .uri(format!("/confirm?id={}&token={}", response_data.id, token).as_str())
        .param("id", response_data.id.to_string())
        .param("token", token)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    //Request confirmation
    let confirmation_token_expires_at = chrono::Utc::now() - chrono::Duration::days(1);
    let _ = sqlx::query("UPDATE users SET confirmed_at = NULL, confirmation_token_expires_at = $1 WHERE id = $2")
        .bind(confirmation_token_expires_at)
        .bind(response_data.id)
        .execute(&pool)
        .await
        .expect("Failed to reset confirmation");

    let req = test::TestRequest::post()
        .uri(format!("/request-confirmation-token/{}", response_data.id).as_str())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}
