use actix_web::{App, test};
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Data;
use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;
use metan::http::handlers::operations::create::create_operation;
use metan::services::jwt::Claims;
use metan::services::jwt::JwtService;
use metan::test_utils::environment::Environment;

#[actix_rt::test]
async fn test_create_operation() {
    let environment = Environment::new();
    let (service_container, event_bus) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(create_operation)
    ).await;

    let jwt_service = service_container.jwt_service();
    let claims = Claims::new(
        Uuid::new_v4().to_string(),
        Utc::now().timestamp() as usize + Duration::days(30).num_seconds() as usize,
        format!("{}@example.com", Uuid::new_v4().to_string()),
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
                "kind": "Expense",
                "category_id": None::<Uuid>,
                "category_name": "Food",
                "amount": 100.0,
                "currency": "USD",
                "currency_amount": 100.0,
                "rate": 1.0,
                "label": "Lunch",
                "tags": [
                    {
                        "id": None::<Uuid>,
                        "name": "restaurant"
                    },
                    {
                        "id": None::<Uuid>,
                        "name": "fastfood"
                    }
                ]
            }
        )).to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}