use actix_web::{App, test};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use actix_web::http::header::AUTHORIZATION;
use serde_json::json;
use uuid::Uuid;
use metan::events::event::Event;
use metan::events::event::Event::CategoryEvent;
use metan::features::operations::domain::events::category_creation_requested::CategoryCreationRequested;
use metan::features::operations::domain::events::operation_event::OperationEvent::CategoryCreationRequested as CategoryCreationRequestedEvent;
use metan::support::id::Id;
use metan::http::handlers::categories::create::create_category;
use metan::services::jwt::Claims;
use metan::services::jwt::JwtService;
use metan::test_utils::environment::Environment;

#[actix_rt::test]
async fn test_account_creation_success() {
    let environment = Environment::new();
    let (service_container, event_bus, _) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(create_category)
    ).await;

    let jwt_service = service_container.jwt_service();
    let claims = Claims::new(
        Uuid::new_v4().to_string(),
        Utc::now().timestamp() as usize + Duration::days(30).num_seconds() as usize,
        format!("{}@example.com", Uuid::new_v4()),
    );
    let token = jwt_service.create(claims).expect("Failed to create token");

    let req = test::TestRequest::post()
        .uri("/create")
        .insert_header((
            AUTHORIZATION,
            format!("Bearer {}", token)
        ))
        .set_json(&json!(
            {
                "name": "Test",
                "amount": 100.0,
                "currency": "USD",
                "currency_amount": 100.0,
                "rate": 1.0,
                "icon": "icon",
                "source": "Kaspi1"
            }
        )).to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), 200);
}