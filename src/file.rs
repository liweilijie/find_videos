use chrono::Utc;
use crate::util::uuid_v4;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::FromRow)]
pub struct File {
    pub id: String,
    pub disk_name: String,
    pub file_name: String,
    pub dir: bool,
    pub hostname: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl File {
   pub fn new(
       disk_name: String,
       file_name: String,
       dir: bool,
       hostname: Option<String>,
   ) -> Self {
       let hostname = hostname.unwrap_or_else(|| format!("{}:{}", whoami::hostname(), whoami::username()));
       Self {
           id: uuid_v4(),
           disk_name,
           file_name,
           dir,
           timestamp: Utc::now(),
           hostname
       }
   }
}