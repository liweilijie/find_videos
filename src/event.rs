use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::file::File;
use crate::util::uuid_v4;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    Create,
    Delete,
}

pub struct Event {
    pub id: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub hostname: String,
    pub event_type: EventType,
    pub file_name: String,
    pub file_id: String,
}

impl Event {
    pub fn new_create(f: &File) -> Event {
        Event {
            id: uuid_v4(),
            timestamp: f.timestamp,
            hostname: f.hostname.clone(),
            event_type: EventType::Create,
            file_name: f.file_name.clone(),
            file_id: f.id.clone(),
        }
    }

    pub fn new_delete(file_id: &str, file_name: &str) -> Event {
        let hostname = format!("{}:{}", whoami::hostname(), whoami::username());

        Event {
            id: uuid_v4(),
            timestamp: chrono::Utc::now(),
            hostname,
            event_type: EventType::Delete,
            file_id: file_id.to_string(),
            file_name: file_name.to_string(),
        }
    }
}