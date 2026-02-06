//! Topic index validation.
//!
//! Loads and validates the topic index from TOML files in `data/topics/`.

use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A search index entry for client-side fuzzy search.
#[derive(Debug, Clone, Serialize)]
pub struct SearchEntry {
    /// Unique identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Searchable text (name + aliases joined).
    pub searchable: String,
    /// Category (crate, book, std).
    pub category: String,
    /// Brief description.
    pub brief: String,
    /// Path for linking (API docs for crates, book path for books).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// A complete topic index loaded from multiple TOML files.
#[derive(Debug, Default)]
pub struct TopicIndex {
    pub categories: HashMap<String, Category>,
    pub topics: HashMap<String, Topic>,
}

/// A topic category for faceted search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub description: String,
}

/// A single topic entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub category: String,
    pub brief: String,
}

/// Intermediate structure for deserializing categories.toml.
#[derive(Debug, Deserialize)]
struct CategoriesFile {
    categories: HashMap<String, Category>,
}

/// Intermediate structure for deserializing topic files.
#[derive(Debug, Deserialize)]
struct TopicsFile {
    topics: HashMap<String, Topic>,
}

/// Validation error details.
#[derive(Debug)]
pub struct ValidationError {
    pub topic_id: String,
    pub message: String,
}

/// Result of validating the topic index.
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
    pub stats: ValidationStats,
}

/// Statistics about the topic index.
#[derive(Debug, Default)]
pub struct ValidationStats {
    pub topic_count: usize,
    pub category_count: usize,
    pub alias_count: usize,
}

impl TopicIndex {
    /// Load the topic index from a directory of TOML files.
    pub fn load(dir: &Path) -> AnyResult<Self> {
        let mut index = TopicIndex::default();

        // Load categories first.
        let categories_path = dir.join("categories.toml");
        if categories_path.exists() {
            let contents = std::fs::read_to_string(&categories_path)
                .with_context(|| format!("reading {}", categories_path.display()))?;
            let file: CategoriesFile = toml::from_str(&contents)
                .with_context(|| format!("parsing {}", categories_path.display()))?;
            index.categories = file.categories;
        }

        // Load all other TOML files as topic files.
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "toml") {
                let filename = path.file_name().unwrap().to_string_lossy();
                if filename == "categories.toml" {
                    continue;
                }

                let contents = std::fs::read_to_string(&path)
                    .with_context(|| format!("reading {}", path.display()))?;
                let file: TopicsFile = toml::from_str(&contents)
                    .with_context(|| format!("parsing {}", path.display()))?;

                for (id, topic) in file.topics {
                    if index.topics.contains_key(&id) {
                        bail!("duplicate topic id '{}' in {}", id, path.display());
                    }
                    index.topics.insert(id, topic);
                }
            }
        }

        Ok(index)
    }

    /// Validate the topic index for consistency.
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::default();

        // Collect category IDs for reference checking.
        let category_ids: HashSet<&str> = self.categories.keys().map(|s| s.as_str()).collect();

        for (id, topic) in &self.topics {
            // Validate topic ID format (kebab-case).
            if !is_valid_id(id) {
                result.errors.push(ValidationError {
                    topic_id: id.clone(),
                    message: format!("invalid topic id '{}': must be lowercase kebab-case", id),
                });
            }

            // Validate category exists.
            if !category_ids.contains(topic.category.as_str()) {
                result.errors.push(ValidationError {
                    topic_id: id.clone(),
                    message: format!("unknown category '{}'", topic.category),
                });
            }

            // Count aliases.
            result.stats.alias_count += topic.aliases.len();

            // Validate brief is not empty.
            if topic.brief.is_empty() {
                result.errors.push(ValidationError {
                    topic_id: id.clone(),
                    message: "brief description is empty".to_string(),
                });
            }

            // Validate name is not empty.
            if topic.name.is_empty() {
                result.errors.push(ValidationError {
                    topic_id: id.clone(),
                    message: "name is empty".to_string(),
                });
            }
        }

        // Update stats.
        result.stats.topic_count = self.topics.len();
        result.stats.category_count = self.categories.len();

        result
    }
}

/// Check if a string is a valid topic ID (lowercase kebab-case).
fn is_valid_id(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '-' {
            // Hyphen cannot be at start/end or consecutive.
            if chars.peek().map_or(true, |&next| next == '-') {
                return false;
            }
        } else if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
            return false;
        }
    }

    !s.starts_with('-') && !s.ends_with('-')
}

impl ValidationResult {
    /// Returns true if there are no errors (warnings are ok).
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Print a report of the validation results.
    pub fn print_report(&self) {
        println!("Topic Index Validation Report");
        println!("=============================");
        println!();
        println!("Statistics:");
        println!("  Topics:     {}", self.stats.topic_count);
        println!("  Categories: {}", self.stats.category_count);
        println!("  Aliases:    {}", self.stats.alias_count);
        println!();

        if self.errors.is_empty() && self.warnings.is_empty() {
            println!("No errors or warnings.");
        }

        if !self.errors.is_empty() {
            println!("Errors ({}):", self.errors.len());
            for err in &self.errors {
                println!("  [{}] {}", err.topic_id, err.message);
            }
            println!();
        }

        if !self.warnings.is_empty() {
            println!("Warnings ({}):", self.warnings.len());
            for warn in &self.warnings {
                println!("  [{}] {}", warn.topic_id, warn.message);
            }
            println!();
        }

        if self.is_ok() {
            println!("Validation passed.");
        } else {
            println!("Validation failed with {} error(s).", self.errors.len());
        }
    }
}

impl TopicIndex {
    /// Print a summary of the topic index.
    pub fn print_summary(&self, verbose: bool) {
        // Group topics by category.
        let mut by_category: HashMap<&str, Vec<(&str, &Topic)>> = HashMap::new();
        for (id, topic) in &self.topics {
            by_category
                .entry(topic.category.as_str())
                .or_default()
                .push((id.as_str(), topic));
        }

        // Sort topics within each category.
        for topics in by_category.values_mut() {
            topics.sort_by_key(|(id, _)| *id);
        }

        // Print summary.
        println!("Topic Index Summary");
        println!("===================\n");

        // Categories section.
        println!("Categories:");
        let mut cat_order: Vec<_> = self.categories.keys().collect();
        cat_order.sort();
        for cat_id in cat_order {
            let cat = &self.categories[cat_id];
            let count = by_category.get(cat_id.as_str()).map_or(0, |v| v.len());
            println!("  {} ({}) - {}", cat.name, count, cat.description);

            if verbose {
                if let Some(topics) = by_category.get(cat_id.as_str()) {
                    for (id, topic) in topics {
                        println!("    - {} ({})", topic.name, id);
                    }
                }
            }
        }
        println!();

        // Totals.
        let total_aliases: usize = self.topics.values().map(|t| t.aliases.len()).sum();
        println!("Totals:");
        println!("  {} topics", self.topics.len());
        println!("  {} categories", self.categories.len());
        println!("  {} aliases", total_aliases);
    }

    /// Export the topic index as a search index for client-side fuzzy search.
    pub fn export_search_index(&self) -> Vec<SearchEntry> {
        let mut entries = Vec::new();

        for (id, topic) in &self.topics {
            // Build searchable text from name + aliases, pipe-separated.
            // The pipe delimiter preserves multi-word alias boundaries
            // so the search algorithm can match each alias individually.
            let searchable = std::iter::once(topic.name.as_str())
                .chain(topic.aliases.iter().map(|s| s.as_str()))
                .collect::<Vec<_>>()
                .join("|");

            // Generate path based on category (relative, no leading slash).
            let path = match topic.category.as_str() {
                "crate" => {
                    // Convert topic id to crate name.
                    // Strip "-crate" suffix if present, then convert hyphens to underscores.
                    let base_id = id.strip_suffix("-crate").unwrap_or(id);
                    let crate_name = base_id.replace('-', "_");
                    Some(format!("api/rustmax/{}/index.html", crate_name))
                }
                "book" => {
                    // Book path (e.g., "trpl" -> "library/trpl/").
                    Some(format!("library/{}/", id))
                }
                "std" => {
                    // Std module path (e.g., "std-sync-atomic" -> "api/std/sync/atomic/index.html").
                    let module_path = id.strip_prefix("std-").unwrap_or(id).replace('-', "/");
                    Some(format!("api/std/{}/index.html", module_path))
                }
                _ => None,
            };

            entries.push(SearchEntry {
                id: id.clone(),
                name: topic.name.clone(),
                searchable,
                category: topic.category.clone(),
                brief: topic.brief.clone(),
                path,
            });
        }

        // Sort entries by category, then by name.
        entries.sort_by(|a, b| {
            a.category
                .cmp(&b.category)
                .then_with(|| a.name.cmp(&b.name))
        });

        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ids() {
        assert!(is_valid_id("ownership"));
        assert!(is_valid_id("async-await"));
        assert!(is_valid_id("e0382"));
        assert!(is_valid_id("serde-json"));

        assert!(!is_valid_id(""));
        assert!(!is_valid_id("-foo"));
        assert!(!is_valid_id("foo-"));
        assert!(!is_valid_id("foo--bar"));
        assert!(!is_valid_id("Foo"));
        assert!(!is_valid_id("foo_bar"));
        assert!(!is_valid_id("foo bar"));
    }

    #[test]
    fn test_search_index_contains_aliases() {
        let mut index = TopicIndex::default();
        index.topics.insert(
            "trpl".to_string(),
            Topic {
                name: "The Rust Programming Language".to_string(),
                aliases: vec![
                    "TRPL".to_string(),
                    "The Book".to_string(),
                ],
                category: "book".to_string(),
                brief: "The official Rust book".to_string(),
            },
        );

        let entries = index.export_search_index();
        let trpl_entry = entries.iter().find(|e| e.id == "trpl").unwrap();

        // Verify searchable text contains name and all aliases.
        assert!(trpl_entry.searchable.contains("The Rust Programming Language"));
        assert!(trpl_entry.searchable.contains("TRPL"));
        assert!(trpl_entry.searchable.contains("The Book"));

        // Verify path is generated correctly for books.
        assert_eq!(trpl_entry.path, Some("library/trpl/".to_string()));
    }

    #[test]
    fn test_trpl_searchable_matches_trpl_query() {
        let mut index = TopicIndex::default();
        index.topics.insert(
            "trpl".to_string(),
            Topic {
                name: "The Rust Programming Language".to_string(),
                aliases: vec!["TRPL".to_string()],
                category: "book".to_string(),
                brief: "The official Rust book".to_string(),
            },
        );

        let entries = index.export_search_index();
        let trpl_entry = entries.iter().find(|e| e.id == "trpl").unwrap();

        // The searchable text lowercased should contain "trpl".
        let searchable_lower = trpl_entry.searchable.to_lowercase();
        assert!(
            searchable_lower.contains("trpl"),
            "searchable '{}' should contain 'trpl'",
            trpl_entry.searchable
        );
    }
}
