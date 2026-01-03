//! Anthology: A document publishing platform.
//!
//! A demonstration application for the rustmax crate ecosystem.

use anthology::cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
