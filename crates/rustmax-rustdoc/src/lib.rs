//! Custom rustdoc HTML renderer for rustmax documentation.
//!
//! This crate parses rustdoc JSON output and renders it to custom HTML,
//! providing an alternative frontend to the standard rustdoc output.

mod parse;
mod types;
mod render;
mod output;

pub use parse::load_json;
pub use types::{ModuleTree, RenderableItem};
pub use render::RenderContext;
pub use output::write_docs;

use rmx::prelude::*;
use std::path::{Path, PathBuf};

/// Configuration for documentation rendering.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Output directory for generated HTML.
    pub output_dir: PathBuf,
    /// Base URL for external crate documentation (e.g., "https://docs.rs").
    pub external_base_url: String,
    /// Whether to include private items in the documentation.
    pub include_private: bool,
    /// Crate version to display.
    pub crate_version: Option<String>,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("target/rmxdoc"),
            external_base_url: "https://docs.rs".to_string(),
            include_private: false,
            crate_version: None,
        }
    }
}

/// Main documentation builder.
pub struct RustDoc {
    /// The parsed rustdoc JSON crate data.
    pub krate: rustdoc_types::Crate,
    /// Rendering configuration.
    pub config: RenderConfig,
}

impl RustDoc {
    /// Load documentation from a rustdoc JSON file.
    pub fn from_json(path: &Path) -> AnyResult<Self> {
        let krate = parse::load_json(path)?;
        Ok(Self {
            krate,
            config: RenderConfig::default(),
        })
    }

    /// Load documentation from JSON bytes.
    pub fn from_bytes(json: &[u8]) -> AnyResult<Self> {
        let krate = parse::load_bytes(json)?;
        Ok(Self {
            krate,
            config: RenderConfig::default(),
        })
    }

    /// Set the output directory.
    pub fn output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config.output_dir = dir.into();
        self
    }

    /// Set the external documentation base URL.
    pub fn external_base_url(mut self, url: impl Into<String>) -> Self {
        self.config.external_base_url = url.into();
        self
    }

    /// Include private items in documentation.
    pub fn include_private(mut self, include: bool) -> Self {
        self.config.include_private = include;
        self
    }

    /// Set the crate version to display.
    pub fn crate_version(mut self, version: impl Into<String>) -> Self {
        self.config.crate_version = Some(version.into());
        self
    }

    /// Render the documentation to HTML.
    pub fn render(&self) -> AnyResult<()> {
        let ctx = render::RenderContext::new(&self.krate, &self.config)?;
        output::write_docs(&ctx)
    }
}
