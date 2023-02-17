mod log;
mod cli;
mod database;
mod file;
mod util;
mod event;

use anyhow::Result;
use tracing::info;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    log::log_init();

    let args = cli::Args::parse();

    info!("start find videos and args:{:?}.", args);

    Ok(())
}
