//! Error types for Anthology.

use thiserror::Error;
use std::path::PathBuf;

/// The result type for Anthology operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in Anthology.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Failed to parse config file {path}: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: rustmax::toml::de::Error,
    },

    #[error("Collection not found at {path}")]
    CollectionNotFound { path: PathBuf },

    #[error("Document error in {path}: {message}")]
    Document { path: PathBuf, message: String },

    #[error("Frontmatter parse error in {path}: {message}")]
    Frontmatter { path: PathBuf, message: String },

    #[error("Template error: {0}")]
    Template(#[from] rustmax::tera::Error),

    #[error("JSON error: {0}")]
    Json(#[from] rustmax::serde_json::Error),

    #[error("Directory walk error: {0}")]
    WalkDir(#[from] rustmax::walkdir::Error),

    #[error("Ignore pattern error: {0}")]
    Ignore(#[from] rustmax::ignore::Error),

    #[error("Build error: {message}")]
    Build { message: String },

    #[error("Server error: {message}")]
    Server { message: String },

    #[error("Remote fetch error for {url}: {message}")]
    Remote { url: String, message: String },

    #[error("{0}")]
    Other(#[from] rustmax::anyhow::Error),
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
