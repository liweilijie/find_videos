mod cli;
mod database;
mod event;
mod file;
mod log;
mod settings;
mod util;
mod find;
mod scan;

use clap::Parser;
use eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    log::log_init();

    let args = cli::Args::parse();

    info!("start find videos and args:{:?}.", args);

    args.command.run().await
}
