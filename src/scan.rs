use crate::database::Database;
use crate::file::File;
use async_walkdir::{DirEntry, WalkDir};
use clap::Subcommand;
use eyre::Result;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use time::Instant;
use tokio_stream::StreamExt;
use tracing::{debug, error};

const CHANNEL_BUFFER_SIZE: usize = 10000;
const DEFAULT_VOLUMES_PATH: &str = "/Volumes";
const EXCLUDE_MAC_VOLUMES_PATH: &str = "/Volumes/Macintosh";

#[derive(Debug, Subcommand)]
pub enum ScanCommand {
    Scan {
        #[arg(long, short)]
        name: Option<String>,
    },
}

impl ScanCommand {
    pub async fn run(self, db: &mut impl Database) -> Result<()> {
        let start = Instant::now();
        let total_files = Arc::new(AtomicU64::new(0));

        match self {
            Self::Scan { name } => {
                if name.is_some() {
                    debug!("scan name:{name:?}");
                }

                let (tx, mut rx) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);

                let total_files1 = Arc::clone(&total_files);
                tokio::spawn(async move {
                    let mut entries =
                        WalkDir::new(name.unwrap_or(DEFAULT_VOLUMES_PATH.to_string()));
                    loop {
                        match entries.next().await {
                            Some(Ok(entry)) => {
                                if is_hidden(&entry) || is_sys(&entry) {
                                    continue;
                                }

                                // just scan director or .mp4 or .mp3 file.
                                if !entry.path().is_dir() && !is_need_scan(&entry) {
                                    continue;
                                }

                                // debug!("file:{}", entry.path().display());
                                let f = File::new(
                                    entry.path().display().to_string(),
                                    entry.file_name().to_string_lossy().to_string(),
                                    entry.path().is_dir(),
                                    None,
                                );

                                if let Err(e) = tx.send(f).await {
                                    error!("send channel error:{}", e);
                                }

                                // db.save(&f).await?;
                                total_files1.fetch_add(1, Ordering::Relaxed);
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
                });

                while let Some(f) = rx.recv().await {
                    debug!("got file:{}", f.file_name);
                    db.save(&f).await?;
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
        .map(|s| s.starts_with(EXCLUDE_MAC_VOLUMES_PATH))
        .unwrap_or(false)
}

fn is_need_scan(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".mp4") || s.ends_with(".mp3"))
        .unwrap_or(false)
}
