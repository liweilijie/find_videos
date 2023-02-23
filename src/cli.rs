use crate::database::Sqlite;
use crate::find::FindCommand;
use crate::scan::ScanCommand;
use crate::settings::Settings;
use clap::{Parser, Subcommand};
use eyre::{Result, WrapErr};

/// scan or find anything.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// del or trim of subcommand.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
#[command(infer_subcommands = true)]
pub enum Commands {
    /// delete sample file
    // Scan { name: Option<String> },
    #[command(flatten)]
    Scan(ScanCommand),
    /// trim name
    #[command(flatten)]
    Find(FindCommand),
}

impl Commands {
    pub async fn run(self) -> Result<()> {
        let settings = Settings::new().wrap_err("could not load settings.")?;
        let mut db = Sqlite::new(&settings.db_path).await?;

        match self {
            Self::Scan(scan) => scan.run(&mut db).await,
            Self::Find(find) => find.run(&mut db, &settings).await,
        }
    }
}
