use crate::file::File;
use crate::util::uuid_v4;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    Create,
    Delete,
}

#[derive(Debug)]
pub struct Event {
    pub id: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub hostname: String,
    pub event_type: EventType,
    pub full_path: String,
}

impl Event {
    pub fn new_create(f: &File) -> Event {
        Event {
            id: uuid_v4(),
            timestamp: f.timestamp,
            hostname: f.hostname.clone(),
            event_type: EventType::Create,
            full_path: f.full_path.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn new_delete(full_path: &str) -> Event {
        let hostname = format!("{}:{}", whoami::hostname(), whoami::username());

        Event {
            id: uuid_v4(),
            timestamp: chrono::Utc::now(),
            hostname,
            event_type: EventType::Delete,
            full_path: full_path.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::log::log_init;
    use tracing::debug;

    #[test]
    fn test_event() {
        log_init();
        let event = Event::new_delete("/test/rust");
        debug!("event:{event:#?}");
    }
}
