//! Document model and frontmatter parsing.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use rustmax::jiff::civil::Date;
use rustmax::blake3;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// A document in the collection.
#[derive(Debug, Clone)]
pub struct Document {
    /// Path to the source file.
    pub source_path: PathBuf,
    /// Parsed frontmatter.
    pub frontmatter: Frontmatter,
    /// Raw markdown content (after frontmatter).
    pub content: String,
    /// Content hash for caching.
    pub content_hash: String,
}

impl Document {
    /// Parse a document from file contents.
    pub fn parse(source_path: PathBuf, raw: &str) -> Result<Self> {
        let (frontmatter, content) = parse_frontmatter(&source_path, raw)?;

        // Compute content hash.
        let hash = blake3::hash(raw.as_bytes());
        let content_hash = hash.to_hex().to_string();

        Ok(Self {
            source_path,
            frontmatter,
            content,
            content_hash,
        })
    }

    /// Load a document from a file path.
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        Self::parse(path.to_path_buf(), &raw)
    }

    /// Get the URL slug for this document.
    pub fn slug(&self) -> String {
        self.frontmatter.slug.clone().unwrap_or_else(|| {
            self.source_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("untitled")
                .to_string()
        })
    }

    /// Get the URL path for this document.
    pub fn url_path(&self) -> String {
        format!("/{}/", self.slug())
    }

    /// Validate the document.
    pub fn validate(&self) -> Result<()> {
        if self.frontmatter.title.is_empty() {
            return Err(Error::document(&self.source_path, "missing title"));
        }
        Ok(())
    }

    /// Get an excerpt from the content.
    pub fn excerpt(&self, separator: &str, max_chars: usize) -> String {
        if let Some(pos) = self.content.find(separator) {
            self.content[..pos].trim().to_string()
        } else {
            // Take first paragraph or max_chars.
            let content = self.content.trim();
            if let Some(pos) = content.find("\n\n") {
                if pos < max_chars {
                    return content[..pos].to_string();
                }
            }
            if content.len() <= max_chars {
                content.to_string()
            } else {
                // Find word boundary.
                let truncated = &content[..max_chars];
                if let Some(pos) = truncated.rfind(char::is_whitespace) {
                    format!("{}...", &truncated[..pos])
                } else {
                    format!("{}...", truncated)
                }
            }
        }
    }

    /// Convert to exportable format.
    pub fn to_export(&self) -> DocumentExport {
        DocumentExport {
            slug: self.slug(),
            title: self.frontmatter.title.clone(),
            date: self.frontmatter.date.map(|d| d.to_string()),
            tags: self.frontmatter.tags.clone(),
            draft: self.frontmatter.draft,
            content: self.content.clone(),
        }
    }

    /// Get word count.
    pub fn word_count(&self) -> usize {
        use rustmax::unicode_segmentation::UnicodeSegmentation;
        self.content.unicode_words().count()
    }

    /// Estimated reading time in minutes.
    pub fn reading_time(&self) -> usize {
        let words = self.word_count();
        (words / 200).max(1) // Assume 200 WPM.
    }
}

/// Document frontmatter (metadata).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    /// Document title.
    #[serde(default)]
    pub title: String,
    /// Publication date.
    #[serde(default, with = "option_date_format")]
    pub date: Option<Date>,
    /// Tags/categories.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Whether this is a draft.
    #[serde(default)]
    pub draft: bool,
    /// Custom URL slug.
    #[serde(default)]
    pub slug: Option<String>,
    /// Template override.
    #[serde(default)]
    pub template: Option<String>,
    /// Description/summary.
    #[serde(default)]
    pub description: Option<String>,
    /// Author override.
    #[serde(default)]
    pub author: Option<String>,
    /// Extra metadata as key-value pairs.
    #[serde(default, flatten)]
    pub extra: std::collections::HashMap<String, rustmax::toml::Value>,
}

/// Serializable document for export.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentExport {
    pub slug: String,
    pub title: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub content: String,
}

/// Parse frontmatter from document content.
///
/// Supports TOML frontmatter delimited by `---`.
fn parse_frontmatter(path: &Path, raw: &str) -> Result<(Frontmatter, String)> {
    let trimmed = raw.trim_start();

    if !trimmed.starts_with("---") {
        // No frontmatter, use defaults.
        return Ok((Frontmatter::default(), raw.to_string()));
    }

    // Find the closing delimiter.
    let rest = &trimmed[3..];
    let end = rest.find("\n---").ok_or_else(|| {
        Error::frontmatter(path, "unclosed frontmatter (missing closing ---)")
    })?;

    let frontmatter_str = &rest[..end].trim();
    let content = &rest[end + 4..]; // Skip "\n---"

    let frontmatter: Frontmatter =
        rustmax::toml::from_str(frontmatter_str).map_err(|e| {
            Error::frontmatter(path, format!("invalid TOML: {}", e))
        })?;

    Ok((frontmatter, content.trim_start().to_string()))
}

mod option_date_format {
    use rustmax::jiff::civil::Date;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let date: Date = s.parse().map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let raw = r#"---
title = "Hello World"
date = "2024-01-15"
tags = ["rust", "test"]
draft = false
---

This is the content.
"#;

        let doc = Document::parse(PathBuf::from("test.md"), raw).unwrap();
        assert_eq!(doc.frontmatter.title, "Hello World");
        assert_eq!(doc.frontmatter.tags, vec!["rust", "test"]);
        assert!(!doc.frontmatter.draft);
        assert!(doc.content.contains("This is the content"));
    }

    #[test]
    fn test_no_frontmatter() {
        let raw = "Just some content without frontmatter.";
        let doc = Document::parse(PathBuf::from("test.md"), raw).unwrap();
        assert_eq!(doc.frontmatter.title, "");
        assert_eq!(doc.content, raw);
    }

    #[test]
    fn test_word_count() {
        let raw = r#"---
title = "Test"
---

One two three four five.
"#;
        let doc = Document::parse(PathBuf::from("test.md"), raw).unwrap();
        assert_eq!(doc.word_count(), 5);
    }
}
