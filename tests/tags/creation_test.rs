use actix_web::{App, test};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use tokio::time::{self, Duration as TokioDuration};
use actix_web::http::header::AUTHORIZATION;
use serde_json::json;
use uuid::Uuid;
use metan::events::event::Event;
use metan::events::event::Event::TagEvent;
use metan::features::operations::domain::events::tag_creation_requested::TagCreationRequested;
use metan::features::operations::domain::events::operation_event::OperationEvent::TagCreationRequested as TagCreationRequestedEvent;
use metan::support::id::Id;
use metan::http::handlers::tags::create::create_tag;
use metan::services::jwt::Claims;
use metan::services::jwt::JwtService;
use metan::test_utils::environment::Environment;

#[actix_rt::test]
async fn create_tag_http_test() {
    let environment = Environment::new();
    let (service_container, event_bus, _) = environment.setup().await;

    let app = test::init_service(
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(service_container.clone()))
            .app_data(Data::new(event_bus))
            .service(create_tag)
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
                "name": "Food",
            }
        )).to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), 200);
}

#[actix_rt::test]
async fn create_tag_event_listener_test() {
    let environment = Environment::new();
    let (service_container, event_bus, mut response) = environment.setup().await;

    let event = Event::OperationEvent(
        TagCreationRequestedEvent(
            TagCreationRequested::new(
                Id::new(Uuid::new_v4()),
                Id::new(Uuid::new_v4()),
                Id::new(Uuid::new_v4()),
                Id::new(Uuid::new_v4()),
                "Food2".to_string(),
            ),
        )
    );


    event_bus.publish(event).await.expect("Failed to publish event");

    match time::timeout(TokioDuration::from_secs(5), response.recv()).await {
        Ok(Some(responder)) => {
            println!("Responder: {:?}", responder);
            let result = responder.handle().await;
            assert!(result.is_ok());
        },
        Ok(None) => panic!("Channel closed unexpectedly"),
        Err(_) => panic!("Test timed out waiting for the response"),
    }

}