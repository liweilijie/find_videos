use crate::event::{Event, EventType};
use crate::file::File;
use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteRow},
    Result, Row,
};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use chrono::{TimeZone, Utc};
use tracing::debug;

pub struct Context {
    disk_name: String,
    file_name: String,
    dir: bool,
}

#[async_trait]
pub trait Database: Send + Sync {
    async fn save(&mut self, f: &File) -> Result<()>;
    async fn save_bulk(&mut self, f: &[File]) -> Result<()>;
    async fn update(&self, h: &File) -> Result<()>;
    async fn file_count(&self) -> Result<i64>;
    async fn event_count(&self) -> Result<i64>;
    async fn query_file(&self, query: &str) -> Result<Vec<File>>;
}

pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        debug!("opening sqlite database at {:?}", path);

        let create = !path.exists();
        if create {
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }
        }

        let opts = SqliteConnectOptions::from_str(path.as_os_str().to_str().unwrap())?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new().connect_with(opts).await?;

        Self::setup_db(&pool).await?;

        Ok(Self { pool })
    }

    async fn setup_db(pool: &SqlitePool) -> Result<()> {
        debug!("running sqlite database setup.");

        sqlx::migrate!("./migrations").run(pool).await?;

        Ok(())
    }

    async fn save_event(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, e: &Event) -> Result<()> {
        let event_type = match e.event_type {
            EventType::Create => "create",
            EventType::Delete => "delete",
        };

        sqlx::query("insert or ignore into events(id, timestamp, hostname, event_type, file_id, file_name) values(?1, ?2, ?3, ?4, ?5)")
            .bind(e.id.as_str())
            .bind(e.timestamp.timestamp_nanos())
            .bind(e.hostname.as_str())
            .bind(event_type)
            .bind(e.file_id.as_str())
            .bind(e.file_name.as_str())
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn save_raw(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, f: &File) -> Result<()> {
        sqlx::query(
            "insert or ignore into file(id, timestamp, full_path, file_name, hostname)\
                 values(?1, ?2, ?3, ?4, ?5)",
        )
        .bind(f.id.as_str())
        .bind(f.timestamp.timestamp_nanos())
        .bind(f.full_path.as_str())
        .bind(f.file_name.as_str())
        .execute(tx)
        .await?;

        Ok(())
    }

    fn query_file(row: SqliteRow) -> File {
        File {
            id: row.get("id"),
            timestamp: Utc.timestamp_nanos(row.get("timestamp")),
            full_path: row.get("full_path"),
            file_name: row.get("file_name"),
            hostname: row.get("hostname"),
        }
    }
}

#[async_trait]
impl Database for Sqlite {
    async fn save(&mut self, f: &File) -> Result<()> {
        debug!("saving file to sqlite");
        let event = Event::new_create(f);

        let mut tx = self.pool.begin().await?;
        Self::save_raw(&mut tx, f).await?;
        Self::save_event(&mut tx, &event).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn save_bulk(&mut self, f: &[File]) -> Result<()> {
        debug!("saving file to sqlite on bulk.");

        let mut tx = self.pool.begin().await?;
        for i in f {
            let event = Event::new_create(i);
            Self::save_raw(&mut tx, i).await?;
            Self::save_event(&mut tx, &event).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn update(&self, f: &File) -> Result<()> {
        debug!("updating sqlite file.");
        sqlx::query(
            "update file set timestamp = ?2, full_path= ?3, file_name = ?4, hostname = ?5 where id = ?1",
        )
            .bind(f.id.as_str())
            .bind(f.timestamp.timestamp_nanos())
            .bind(f.full_path.as_str())
            .bind(f.file_name.as_str())
            .bind(f.hostname.as_str())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn file_count(&self) -> Result<i64> {
        let res: (i64,) = sqlx::query_as("select count(1) from file")
            .fetch_one(&self.pool)
            .await?;

        Ok(res.0)
    }

    async fn event_count(&self) -> Result<i64> {
        let res: i64 = sqlx::query_scalar("select count(1) from events")
            .fetch_one(&self.pool)
            .await?;

        Ok(res)
    }

    async fn query_file(&self, query: &str) -> Result<Vec<File>> {
        let res: Vec<File> = sqlx::query(query)
            .map(Self::query_file)
            .fetch_all(&self.pool)
            .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::util::uuid_v4;
    use super::*;

    async fn db_save(db: &mut impl Database, f: &File) -> Result<()> {
        db.save(f).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_db() {
       let mut db = Sqlite::new("./sofaraway.sqlite").await.unwrap();
        let f = File {
            id: uuid_v4(),
            timestamp: Utc::now(),
            hostname: "liweideMacBook-Pro.local".to_string(),
            full_path: "/Users/liwei/coding/rust/tools/find_videos".to_string(),
            file_name: "find_videos".to_string(),
        };

        db_save(&mut db, &f).await.unwrap();
    }
}
