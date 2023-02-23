use std::io::ErrorKind;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use time::Instant;
use clap::Subcommand;
use crate::database::Database;
use eyre::Result;
use tracing::{debug, error};
use crate::file::File;
use tokio_stream::StreamExt;
use async_walkdir::{DirEntry, WalkDir};

#[derive(Debug, Subcommand)]
pub enum ScanCommand {
    Scan {
        #[arg(long, short)]
        name: Option<String>,
    }
}

impl ScanCommand {
    pub async fn run(self, db: &mut impl Database) -> Result<()> {
        let start = Instant::now();
        let total_files = Arc::new(AtomicU64::new(0));

        match self {
            Self::Scan {name} => {
                debug!("scan name:{name:?}");
                let mut entries = WalkDir::new(name.unwrap_or("/Volumes/".to_string()));
                loop {
                    match entries.next().await {
                        Some(Ok(entry)) => {
                            if is_hidden(&entry) || is_sys(&entry) {
                                continue;
                            }
                            debug!("file:{}", entry.path().display());
                            let f = File::new(
                                entry.path().display().to_string(),
                                entry.file_name().to_string_lossy().to_string(),
                                None,
                            );
                            db.save(&f).await?;
                            total_files.fetch_add(1, Ordering::Relaxed);
                        }
                        Some(Err(e)) => match e.kind() {
                            ErrorKind::PermissionDenied => {
                                error!("continue error:{}", e);
                                continue;
                            }
                            other_error => {
                                error!("error:{}", other_error);
                                break;
                            }
                        },
                        None => {
                            debug!("None");
                            break;
                        }
                    }
                }
                debug!(
                    "scan total:{}, elapsed:{}",
                    total_files.load(Ordering::Relaxed),
                    start.elapsed()
                );


            }
        }

        Ok(())
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_sys(entry: &DirEntry) -> bool {
    entry
        .path()
        .to_str()
        .map(|s| s.starts_with("/Volumes/Macintosh"))
        .unwrap_or(false)
}
