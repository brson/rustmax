//! A static, cross-referenced source browser for cargo workspaces.
//!
//! The browser mirrors the workspace as a tree of HTML pages,
//! one per text file plus a listing per directory.
//! Rust source is highlighted and every identifier is linked:
//! references go to definitions in the workspace or to external documentation,
//! and definitions open a list of their references.
//!
//! Cross-references come from rust-analyzer's SCIP output
//! and workspace structure from `cargo metadata`.
//! Per-symbol data is written to sharded files loaded on demand,
//! so pages stay small and the site works from `file://` URLs.

mod files;
mod highlight;
mod index;
mod metadata;
mod render;
mod symbol;

use rmx::prelude::*;
use std::path::{Path, PathBuf};

pub use render::Stats;

/// Options for building a browser site.
#[derive(Debug, Clone)]
pub struct Config {
    /// Any directory inside the workspace.
    pub workspace: PathBuf,
    /// Output directory, defaulting to `target/rmxbrowse`.
    pub output: Option<PathBuf>,
    /// An existing SCIP index to use instead of running rust-analyzer.
    pub scip: Option<PathBuf>,
}

/// Build the browser site, returning where it was written and what it contains.
pub fn build(config: &Config) -> AnyResult<(PathBuf, Stats)> {
    let metadata = metadata::Metadata::load(&config.workspace)?;
    let root = &metadata.workspace_root;
    let output = config.output.clone().unwrap_or_else(|| render::default_output(&metadata));

    let scip_path = match &config.scip {
        Some(path) => path.clone(),
        None => {
            let path = metadata.target_directory.join("rmxbrowse-scip").join("index.scip");
            eprintln!("Indexing {} with rust-analyzer...", root.display());
            index::run_rust_analyzer(root, &path)?;
            path
        }
    };
    let index = index::Index::load(&scip_path)?;

    let excludes = [output.clone(), metadata.target_directory.clone()];
    let files = files::walk(root, &excludes)?;

    eprintln!("Rendering {} files to {}...", files.len(), output.display());
    if output.exists() {
        clear_output(&output)?;
    }
    let site = render::Site { metadata: &metadata, index: &index, files: &files, output: &output };
    let stats = render::render(&site)?;
    Ok((output, stats))
}

/// Remove a previous build, refusing to delete a directory that does not look like one.
fn clear_output(output: &Path) -> AnyResult<()> {
    if !output.join("_rmx").join("browse.js").exists() {
        bail!(
            "output directory {} exists and is not a previous browser build; remove it or choose another",
            output.display(),
        );
    }
    std::fs::remove_dir_all(output)
        .with_context(|| format!("failed to remove {}", output.display()))
}
