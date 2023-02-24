use crate::database::Database;
use crate::settings::Settings;
use clap::Subcommand;
use eyre::Result;
use tracing::info;

#[derive(Debug, Subcommand)]
pub enum FindCommand {
    Find {
        #[arg(long, short)]
        name: String,
        #[arg(long, short = 'p')]
        show_path: bool,
        #[arg(long, short = 'd')]
        only_show_dir: bool,
    },
    Count,
}

impl FindCommand {
    pub async fn run(self, db: &mut impl Database, settings: &Settings) -> Result<()> {
        match self {
            Self::Find {
                name,
                show_path,
                only_show_dir,
            } => {
                let query = if only_show_dir {
                    format!("select * from file where file_name like '%{name}%' and dir = 1;")
                } else {
                    format!("select * from file where file_name like '%{name}%';")
                };
                info!("query:{query}");
                let files = db.query_file(&query).await?;
                for f in &files {
                    if !show_path {
                        println!("{}", f.file_name);
                    } else {
                        println!("{}:({})", f.file_name, f.full_path);
                    }
                }
                // info!("{:?}", files);
            }
            Self::Count => {
                let file_count = db.file_count().await?;
                let event_count = db.event_count().await?;
                println!(
                    "file.count: {}, events.count: {} and in db path: {}",
                    file_count, event_count, settings.db_path
                );
            }
        }

        Ok(())
    }
}
