use rmx::prelude::*;
use std::path::PathBuf;

/// A single doctest example extracted from a markdown file.
#[derive(Debug, Clone)]
pub struct DocTest {
    /// Source markdown file.
    pub source_file: PathBuf,
    /// Starting line number in the markdown file.
    pub line: usize,
    /// Test name (derived from context or line number).
    pub name: String,
    /// The Rust code to test.
    pub code: String,
    /// Whether the test has the `no_run` modifier.
    pub no_run: bool,
    /// Whether the test has the `ignore` modifier.
    pub ignore: bool,
}

/// Configuration for running doctests.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Directory containing markdown files with examples.
    pub doc_dir: PathBuf,
    /// Working directory for generated test crate.
    pub work_dir: PathBuf,
    /// Force rebuild even if up to date.
    pub rebuild: bool,
    /// Arguments to pass to cargo test.
    pub test_args: Vec<String>,
}
