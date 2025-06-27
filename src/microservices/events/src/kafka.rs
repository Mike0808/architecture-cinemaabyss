use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use futures::StreamExt;
use log::{info, error};
use crate::models::Event;
use std::time::Duration;

const KAFKA_BROKERS: &str = "localhost:9092";
const TOPIC: &str = "events";

pub async fn produce_event(event: Event) -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKERS)
        .set("message.timeout.ms", "5000")
        .create()?;

    let payload = serde_json::to_string(&event)?;
    let key = format!("{:?}", event.event_type);

    let record = FutureRecord::to(TOPIC)
        .payload(&payload)
        .key(&key);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(_) => info!("Event produced: {:?}", event),
        Err((e, _)) => error!("Failed to produce event: {}", e),
    }

    Ok(())
}

pub async fn consume_events() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "events-service")
        .set("bootstrap.servers", KAFKA_BROKERS)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[TOPIC])
        .expect("Can't subscribe to specified topic");

    info!("Consumer started, waiting for messages...");

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    match serde_json::from_slice::<Event>(payload) {
                        Ok(event) => info!("Consumed event: {:?}", event),
                        Err(e) => error!("Error deserializing event: {}", e),
                    }
                }
            }
            Err(e) => error!("Kafka error: {}", e),
        }
    }
}