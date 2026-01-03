//! Directory scanning for document discovery.

use rustmax::prelude::*;
use rustmax::ignore::WalkBuilder;
use rustmax::rayon::prelude::*;
use rustmax::log::debug;
use std::path::{Path, PathBuf};

use super::Document;
use crate::Result;

/// Scanner for finding documents in a content directory.
pub struct Scanner {
    root: PathBuf,
}

impl Scanner {
    /// Create a new scanner for the given content directory.
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    /// Scan for all markdown documents.
    pub fn scan(&self) -> Result<Vec<Document>> {
        let paths = self.find_markdown_files()?;

        debug!("Found {} markdown files", paths.len());

        // Parse documents in parallel.
        let documents: Result<Vec<Document>> = paths
            .par_iter()
            .map(|path| {
                debug!("Loading document: {}", path.display());
                Document::load(path)
            })
            .collect();

        documents
    }

    /// Find all markdown files in the content directory.
    fn find_markdown_files(&self) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();

        for entry in WalkBuilder::new(&self.root)
            .hidden(false)
            .git_ignore(true)
            .git_global(false)
            .git_exclude(false)
            .build()
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "markdown" {
                        paths.push(path.to_path_buf());
                    }
                }
            }
        }

        // Sort for deterministic ordering.
        paths.sort();

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax::tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_scanner() {
        let dir = TempDir::new().unwrap();
        let content_dir = dir.path().join("content");
        fs::create_dir(&content_dir).unwrap();

        // Create test documents.
        fs::write(
            content_dir.join("post1.md"),
            "---\ntitle = \"Post 1\"\n---\nContent 1",
        )
        .unwrap();
        fs::write(
            content_dir.join("post2.md"),
            "---\ntitle = \"Post 2\"\n---\nContent 2",
        )
        .unwrap();

        // Create a subdirectory with more documents.
        let subdir = content_dir.join("nested");
        fs::create_dir(&subdir).unwrap();
        fs::write(
            subdir.join("post3.md"),
            "---\ntitle = \"Post 3\"\n---\nContent 3",
        )
        .unwrap();

        let scanner = Scanner::new(&content_dir);
        let docs = scanner.scan().unwrap();

        assert_eq!(docs.len(), 3);
    }
}
