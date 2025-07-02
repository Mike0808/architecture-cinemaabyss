mod api;
mod kafka;
mod models;

use log::{error, info};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting events service");

     // Use tokio::select! to run both tasks concurrently
    tokio::select! {
        _ = kafka::consume_events() => {
            error!("Kafka consumer stopped unexpectedly");
        }
        result = api::run_api() => {
            if let Err(e) = result {
                error!("API error: {}", e);
            }
        }
    }

    Ok(())
}
