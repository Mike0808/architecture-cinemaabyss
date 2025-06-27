use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;
use crate::models::{Event, EventType};
use crate::kafka::produce_event;
use log::info;

pub async fn create_user() -> impl Responder {
    info!("Creating user event");
    let event = Event::new(EventType::User, json!({"action": "created"}));
    produce_event(event).await.unwrap();
    HttpResponse::Created().json(json!({"status": "success"}))
}

pub async fn create_payment() -> impl Responder {
    info!("Creating payment event");
    let event = Event::new(EventType::Payment, json!({"amount": 100, "currency": "USD"}));
    produce_event(event).await.unwrap();
    HttpResponse::Created().json(json!({"status": "success"}))
}

pub async fn create_movie() -> impl Responder {
    info!("Creating movie event");
    let event = Event::new(EventType::Movie, json!({"title": "Inception", "year": 2010}));
    produce_event(event).await.unwrap();
    HttpResponse::Created().json(json!({"status": "success"}))
}
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": true,
        "service": "events-api"
    }))
}
pub async fn run_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/events/user", web::post().to(create_user))
            .route("/api/events/payment", web::post().to(create_payment))
            .route("/api/events/movie", web::post().to(create_movie))
            .route("/api/events/health", web::get().to(health_check)) 
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}