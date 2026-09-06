//! Collection management - loading and representing document collections.

mod config;
mod document;
mod scanner;

pub use config::{
    ANTHOLOGY_VERSION, BuildConfig, CollectionConfig, Config, ConfigFormat, ContentConfig,
    HighlightConfig, ServerConfig,
};
pub use document::{Document, Frontmatter};
pub use scanner::Scanner;

use rmx::prelude::*;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// A collection of documents.
#[derive(Debug)]
pub struct Collection {
    /// Root path of the collection.
    pub root: PathBuf,
    /// All documents in the collection.
    pub documents: Vec<Document>,
}

/// The queries a collection answers, on any slice of documents.
///
/// `Collection` is not the only thing that holds documents: the dev server
/// hands slices to handlers and the REPL narrows them as the user filters. The
/// queries belong to the documents rather than to the container, so they live
/// here and `Collection` delegates.
#[extension_trait]
pub impl DocumentsExt for [Document] {
    /// Non-draft documents, newest first.
    fn published(&self) -> Vec<&Document> {
        self.iter()
            .filter(|d| !d.frontmatter.draft)
            .sorted_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date))
            .collect()
    }

    /// Every document, drafts included, newest first.
    fn all_sorted(&self) -> Vec<&Document> {
        self.iter()
            .sorted_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date))
            .collect()
    }

    /// Documents carrying `tag`.
    fn by_tag(&self, tag: &str) -> Vec<&Document> {
        self.iter()
            .filter(|d| d.frontmatter.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// The document with this slug, if there is one.
    fn by_slug(&self, slug: &str) -> Option<&Document> {
        self.iter().find(|d| d.slug() == slug)
    }

    /// Every tag used, sorted and deduplicated.
    fn tags(&self) -> Vec<String> {
        self.iter()
            .flat_map(|d| d.frontmatter.tags.iter().cloned())
            .sorted()
            .dedup()
            .collect()
    }
}

impl Collection {
    /// Load a collection from a directory.
    pub fn load(root: &Path, _config: &Config) -> Result<Self> {
        let root = root.canonicalize()?;
        let content_dir = root.join("content");

        if !content_dir.exists() {
            return Err(Error::CollectionNotFound { path: root });
        }

        let scanner = Scanner::new(&content_dir);
        let documents = scanner.scan()?;

        Ok(Self { root, documents })
    }

    /// Get all non-draft documents, sorted by date descending.
    pub fn published(&self) -> Vec<&Document> {
        self.documents.published()
    }

    /// Get all documents including drafts, sorted by date descending.
    pub fn all_sorted(&self) -> Vec<&Document> {
        self.documents.all_sorted()
    }

    /// Get documents by tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&Document> {
        self.documents.by_tag(tag)
    }

    /// Get the document with this slug, if there is one.
    pub fn by_slug(&self, slug: &str) -> Option<&Document> {
        self.documents.by_slug(slug)
    }

    /// Get all unique tags.
    pub fn tags(&self) -> Vec<String> {
        self.documents.tags()
    }

    /// Convert to exportable format.
    pub fn to_export(&self) -> CollectionExport {
        CollectionExport {
            documents: self.documents.iter().map(|d| d.to_export()).collect(),
        }
    }
}

/// Serializable collection for export.
#[rmx::derive(Debug, Serialize, Deserialize)]
pub struct CollectionExport {
    pub documents: Vec<document::DocumentExport>,
}
