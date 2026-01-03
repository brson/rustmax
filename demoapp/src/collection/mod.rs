//! Collection management - loading and representing document collections.

mod config;
mod document;
mod scanner;

pub use config::Config;
pub use document::{Document, Frontmatter};
pub use scanner::Scanner;

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
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
        let mut docs: Vec<_> = self
            .documents
            .iter()
            .filter(|d| !d.frontmatter.draft)
            .collect();
        docs.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
        docs
    }

    /// Get all documents including drafts, sorted by date descending.
    pub fn all_sorted(&self) -> Vec<&Document> {
        let mut docs: Vec<_> = self.documents.iter().collect();
        docs.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
        docs
    }

    /// Get documents by tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&Document> {
        self.documents
            .iter()
            .filter(|d| d.frontmatter.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Get all unique tags.
    pub fn tags(&self) -> Vec<String> {
        use rustmax::itertools::Itertools;

        self.documents
            .iter()
            .flat_map(|d| d.frontmatter.tags.iter().cloned())
            .sorted()
            .dedup()
            .collect()
    }

    /// Convert to exportable format.
    pub fn to_export(&self) -> CollectionExport {
        CollectionExport {
            documents: self.documents.iter().map(|d| d.to_export()).collect(),
        }
    }
}

/// Serializable collection for export.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionExport {
    pub documents: Vec<document::DocumentExport>,
}
