use clap::{Parser, Subcommand};

/// scan or find anything.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// del or trim of subcommand.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// delete sample file
    Scan { name: Option<String> },
    /// trim name
    Find { name: String },
}