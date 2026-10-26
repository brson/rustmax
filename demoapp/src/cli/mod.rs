//! Command-line interface for Folio.

mod commands;
pub mod repl;

pub use commands::Cli;

use rmx::clap::Parser as _;

use crate::Result;

/// Run the CLI application.
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
