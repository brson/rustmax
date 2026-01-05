//! Doctest runner for rustmax crate examples.
//!
//! Extracts code examples from markdown files, generates a test crate,
//! and runs the tests with cargo.

mod compile;
mod extract;
mod generate;
mod types;

use rmx::prelude::*;
use std::path::Path;
pub use types::{DocTest, TestConfig};

/// Run doctests from markdown files.
///
/// This is the main entry point for running doctests.
pub fn run_doctests(
    doc_dir: &Path,
    work_dir: &Path,
    test_args: &[String],
    _rebuild: bool,
) -> AnyResult<()> {
    // Extract all examples from markdown files.
    println!("Extracting doctests from {}...", doc_dir.display());
    let examples = extract::extract_all_examples(doc_dir)?;

    if examples.is_empty() {
        println!("No doctests found.");
        return Ok(());
    }

    println!("Found {} doctests.", examples.len());

    // Generate test crate.
    println!("Generating test crate in {}...", work_dir.display());
    generate::generate_test_crate(&examples, work_dir)?;

    // Build test crate.
    println!("Building test crate...");
    compile::build_test_crate(work_dir)?;

    // Run tests.
    println!("Running tests...");
    compile::run_tests(work_dir, test_args)?;

    Ok(())
}
