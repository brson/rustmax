//! Folio: A document publishing platform.
//!
//! A demonstration application for the rustmax crate ecosystem.

use folio::cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
