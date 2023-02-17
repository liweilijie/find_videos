use crate::file::File;
use async_trait::async_trait;
use sql_builder::{esc, quote, SqlBuilder, SqlName};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteRow},
    Result, Row,
};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use tracing::debug;
use crate::event::{Event, EventType};

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
    async fn search(
        &self,
        limit: Option<i64>,
        search_mode: SearchMode,
        filter: FilterMode,
        context: &Context,
        query: &str,
    ) -> Result<Vec<File>>;
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

        sqlx::migrate("./migrations").run(pool).await?;

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
            "insert or ignore into file(id, timestamp, disk_name, file_name, )"
        )
    }
}

#[async_trait]
impl Database for Sqlite {
    async fn save(&mut self, f: &File) -> Result<()> {
        debug!("saving file to sqlite");

        let mut tx = self.pool.begin().await?;
        Self::save_raw(&mut tx, f).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn save_bulk(&mut self, f: &[File]) -> Result<()> {
        todo!()
    }

    async fn update(&self, h: &File) -> Result<()> {
        todo!()
    }

    async fn file_count(&self) -> Result<i64> {
        todo!()
    }

    async fn search(&self, limit: Option<i64>, search_mode: SearchMode, filter: FilterMode, context: &Context, query: &str) -> Result<Vec<File>> {
        todo!()
    }

    async fn query_file(&self, query: &str) -> Result<Vec<File>> {
        todo!()
    }
}
