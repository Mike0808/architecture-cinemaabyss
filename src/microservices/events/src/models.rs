use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    User,
    Payment,
    Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub data: serde_json::Value,
}

impl Event {
    pub fn new(event_type: EventType, data: serde_json::Value) -> Self {
        Self { event_type, data }
    }
}