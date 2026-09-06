//! Error types for Anthology.
//!
//! These use `derive_more` rather than `thiserror`.
//! `thiserror` generates `::thiserror` paths,
//! which resolve only against the extern prelude,
//! so it is the one derive crate that cannot be reached through `rustmax`.
//! `derive_more` covers the same ground and does work through the re-export,
//! which lets Anthology depend on `rustmax` and nothing else.

use rmx::prelude::*;
use std::path::PathBuf;

/// The result type for Anthology operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in Anthology.
#[rmx::derive(Debug, Display, derive_more::Error, From)]
pub enum Error {
    #[display("IO error: {_0}")]
    Io(#[from] std::io::Error),

    #[display("configuration error: {message}")]
    #[from(ignore)]
    Config { message: String },

    #[display("failed to parse config file {}: {source}", path.display())]
    #[from(ignore)]
    ConfigParse {
        path: PathBuf,
        source: rmx::toml::de::Error,
    },

    #[display("failed to parse config file {}: {source}", path.display())]
    #[from(ignore)]
    ConfigParseJson5 {
        path: PathBuf,
        source: rmx::json5::Error,
    },

    #[display("collection not found at {}", path.display())]
    #[from(ignore)]
    CollectionNotFound { path: PathBuf },

    #[display("document error in {}: {message}", path.display())]
    #[from(ignore)]
    Document { path: PathBuf, message: String },

    #[display("frontmatter parse error in {}: {message}", path.display())]
    #[from(ignore)]
    Frontmatter { path: PathBuf, message: String },

    #[display("template error: {_0}")]
    Template(#[from] rmx::tera::Error),

    #[display("JSON error: {_0}")]
    Json(#[from] rmx::serde_json::Error),

    #[display("directory walk error: {_0}")]
    WalkDir(#[from] rmx::walkdir::Error),

    #[display("ignore pattern error: {_0}")]
    Ignore(#[from] rmx::ignore::Error),

    #[display("archive error: {_0}")]
    Zip(#[from] rmx::zip::result::ZipError),

    #[display("build error: {message}")]
    #[from(ignore)]
    Build { message: String },

    #[display("{count} problem{} found", if *count == 1 { "" } else { "s" })]
    #[from(ignore)]
    Check { count: usize },

    #[display("server error: {message}")]
    #[from(ignore)]
    Server { message: String },

    #[display("remote fetch error for {url}: {message}")]
    #[from(ignore)]
    Remote { url: String, message: String },

    #[display("{_0}")]
    Other(#[from] rmx::anyhow::Error),
}

impl Error {
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn document(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::Document {
            path: path.into(),
            message: message.into(),
        }
    }

    pub fn frontmatter(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::Frontmatter {
            path: path.into(),
            message: message.into(),
        }
    }

    pub fn check(count: usize) -> Self {
        Self::Check { count }
    }

    pub fn build(message: impl Into<String>) -> Self {
        Self::Build {
            message: message.into(),
        }
    }

    pub fn server(message: impl Into<String>) -> Self {
        Self::Server {
            message: message.into(),
        }
    }

    pub fn remote(url: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Remote {
            url: url.into(),
            message: message.into(),
        }
    }
}
