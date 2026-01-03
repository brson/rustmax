//! Command-line interface for Anthology.

mod commands;

pub use commands::Cli;

use clap::Parser;
use crate::Result;

/// Run the CLI application.
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
