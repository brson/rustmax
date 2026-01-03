//! Full-text search indexing.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use rustmax::unicode_segmentation::UnicodeSegmentation;
use rustmax::log::info;
use std::collections::HashMap;
use std::path::Path;

use crate::collection::Collection;
use crate::Result;

/// Search index for a collection.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Document entries.
    pub documents: Vec<IndexEntry>,
    /// Inverted index: word -> document indices.
    pub word_index: HashMap<String, Vec<usize>>,
}

/// Entry for a document in the search index.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub slug: String,
    pub title: String,
    pub content_preview: String,
    pub tags: Vec<String>,
    pub word_count: usize,
}

impl SearchIndex {
    /// Build an index from a collection.
    pub fn build(collection: &Collection) -> Self {
        let mut documents = Vec::new();
        let mut word_index: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, doc) in collection.documents.iter().enumerate() {
            // Create entry.
            let entry = IndexEntry {
                slug: doc.slug(),
                title: doc.frontmatter.title.clone(),
                content_preview: doc.excerpt("<!--more-->", 200),
                tags: doc.frontmatter.tags.clone(),
                word_count: doc.word_count(),
            };

            // Index words from title and content.
            let text = format!(
                "{} {} {}",
                doc.frontmatter.title,
                doc.frontmatter.tags.join(" "),
                doc.content
            );

            for word in text.unicode_words() {
                let normalized = word.to_lowercase();
                if normalized.len() >= 2 {
                    word_index
                        .entry(normalized)
                        .or_default()
                        .push(idx);
                }
            }

            documents.push(entry);
        }

        // Deduplicate document indices in word index.
        for indices in word_index.values_mut() {
            indices.sort();
            indices.dedup();
        }

        Self {
            documents,
            word_index,
        }
    }

    /// Search for documents matching a query.
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_words: Vec<String> = query
            .unicode_words()
            .map(|w| w.to_lowercase())
            .filter(|w| w.len() >= 2)
            .collect();

        if query_words.is_empty() {
            return Vec::new();
        }

        // Find documents matching all query words.
        let mut doc_scores: HashMap<usize, usize> = HashMap::new();

        for word in &query_words {
            // Exact match.
            if let Some(indices) = self.word_index.get(word) {
                for &idx in indices {
                    *doc_scores.entry(idx).or_default() += 2;
                }
            }

            // Prefix match.
            for (indexed_word, indices) in &self.word_index {
                if indexed_word.starts_with(word) && indexed_word != word {
                    for &idx in indices {
                        *doc_scores.entry(idx).or_default() += 1;
                    }
                }
            }
        }

        use rustmax::itertools::Itertools;

        // Sort by score descending using itertools.
        doc_scores
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .take(20)
            .filter_map(|(idx, score)| {
                self.documents.get(idx).map(|entry| SearchResult {
                    slug: entry.slug.clone(),
                    title: entry.title.clone(),
                    preview: entry.content_preview.clone(),
                    score,
                })
            })
            .collect()
    }
}

/// A search result.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub preview: String,
    pub score: usize,
}

/// Build and save search index for a collection.
pub fn build_index(collection: &Collection, root: &Path) -> Result<()> {
    info!("Building search index for {} documents", collection.documents.len());

    let index = SearchIndex::build(collection);
    let index_path = root.join("search-index.json");

    let json = rustmax::serde_json::to_string_pretty(&index)?;
    std::fs::write(&index_path, json)?;

    info!("Search index saved to {}", index_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collection::Document;
    use std::path::PathBuf;

    fn make_doc(title: &str, content: &str) -> Document {
        let raw = format!(
            "---\ntitle = \"{}\"\n---\n{}",
            title, content
        );
        Document::parse(PathBuf::from("test.md"), &raw).unwrap()
    }

    #[test]
    fn test_search_index() {
        let docs = vec![
            make_doc("Rust Programming", "Learn about Rust and memory safety."),
            make_doc("Python Basics", "An introduction to Python programming."),
            make_doc("Advanced Rust", "Deep dive into Rust async and lifetimes."),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let index = SearchIndex::build(&collection);
        assert_eq!(index.documents.len(), 3);

        let results = index.search("rust");
        assert_eq!(results.len(), 2);
        assert!(results[0].title.contains("Rust"));

        let results = index.search("python");
        assert_eq!(results.len(), 1);
        assert!(results[0].title.contains("Python"));
    }
}
