mod cli;
mod database;
mod event;
mod file;
mod find;
mod log;
mod scan;
mod settings;
mod util;

use clap::Parser;
use eyre::Result;
// use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    log::log_init();

    let args = cli::Args::parse();

    // info!("start find videos and args:{:?}.", args);

    args.command.run().await
}
