use chrono::Utc;
use crate::util::uuid_v4;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::FromRow)]
pub struct File {
    pub id: String,
    pub full_path: String,
    pub file_name: String,
    pub hostname: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl File {
   pub fn new(
       full_path: String,
       file_name: String,
       hostname: Option<String>,
   ) -> Self {
       let hostname = hostname.unwrap_or_else(|| format!("{}:{}", whoami::hostname(), whoami::username()));
       Self {
           id: uuid_v4(),
           full_path,
           file_name,
           timestamp: Utc::now(),
           hostname
       }
   }
}