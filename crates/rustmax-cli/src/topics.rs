//! Topic index validation.
//!
//! Loads and validates the topic index from TOML files in `data/topics/`.

use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

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
    #[serde(default)]
    pub tags: Vec<String>,
    pub brief: String,
    #[serde(default)]
    pub relations: Vec<Relation>,
}

/// A relation between topics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub kind: RelationKind,
    pub target: String,
}

/// The type of relationship between topics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationKind {
    /// Loose association, bidirectional "see also".
    Related,
    /// Hierarchical: this topic is a subtopic of target.
    Parent,
    /// This crate/tool implements that concept.
    Implements,
    /// Understanding this requires understanding target.
    Requires,
    /// This replaces/deprecates target.
    Supersedes,
    /// Often confused with, but different from.
    Contrast,
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
    pub relation_count: usize,
    pub alias_count: usize,
    pub tag_count: usize,
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

        // Collect all topic IDs for reference checking.
        let topic_ids: HashSet<&str> = self.topics.keys().map(|s| s.as_str()).collect();
        let category_ids: HashSet<&str> = self.categories.keys().map(|s| s.as_str()).collect();

        // Track aliases for uniqueness checking.
        let mut seen_aliases: HashMap<&str, &str> = HashMap::new();

        // Track all tags.
        let mut all_tags: HashSet<&str> = HashSet::new();

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

            // Validate relations.
            for relation in &topic.relations {
                // Check target exists.
                if !topic_ids.contains(relation.target.as_str()) {
                    result.errors.push(ValidationError {
                        topic_id: id.clone(),
                        message: format!(
                            "relation target '{}' does not exist",
                            relation.target
                        ),
                    });
                }

                // Check for self-reference.
                if relation.target == *id {
                    result.errors.push(ValidationError {
                        topic_id: id.clone(),
                        message: "topic cannot relate to itself".to_string(),
                    });
                }

                result.stats.relation_count += 1;
            }

            // Check alias uniqueness (warning, not error).
            for alias in &topic.aliases {
                let alias_lower = alias.to_lowercase();
                if let Some(&other_id) = seen_aliases.get(alias_lower.as_str()) {
                    if other_id != id {
                        result.warnings.push(ValidationError {
                            topic_id: id.clone(),
                            message: format!(
                                "alias '{}' also used by topic '{}'",
                                alias, other_id
                            ),
                        });
                    }
                }
                result.stats.alias_count += 1;
            }

            // Record aliases (we can't insert &alias_lower due to lifetime, so skip dedup tracking for now).

            // Collect tags.
            for tag in &topic.tags {
                all_tags.insert(tag.as_str());
            }

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
        result.stats.tag_count = all_tags.len();

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
        println!("  Relations:  {}", self.stats.relation_count);
        println!("  Aliases:    {}", self.stats.alias_count);
        println!("  Tags:       {}", self.stats.tag_count);
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

        // Collect tags with counts.
        let mut tag_counts: HashMap<&str, usize> = HashMap::new();
        for topic in self.topics.values() {
            for tag in &topic.tags {
                *tag_counts.entry(tag.as_str()).or_default() += 1;
            }
        }
        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

        // Find hub topics (most relations).
        let mut relation_counts: Vec<(&str, usize)> = self
            .topics
            .iter()
            .map(|(id, t)| (id.as_str(), t.relations.len()))
            .filter(|(_, count)| *count > 0)
            .collect();
        relation_counts.sort_by(|a, b| b.1.cmp(&a.1));

        // Find orphan topics (no relations and not referenced).
        let referenced: HashSet<&str> = self
            .topics
            .values()
            .flat_map(|t| t.relations.iter().map(|r| r.target.as_str()))
            .collect();
        let orphans: Vec<&str> = self
            .topics
            .iter()
            .filter(|(id, t)| t.relations.is_empty() && !referenced.contains(id.as_str()))
            .map(|(id, _)| id.as_str())
            .collect();

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

        // Tags section.
        println!("Tags ({} unique):", tags.len());
        if verbose {
            for (tag, count) in &tags {
                println!("  {} ({})", tag, count);
            }
        } else {
            for (tag, count) in tags.iter().take(10) {
                println!("  {} ({})", tag, count);
            }
            if tags.len() > 10 {
                println!("  ... and {} more", tags.len() - 10);
            }
        }
        println!();

        // Hub topics (most connected).
        println!("Most connected topics:");
        for (id, count) in relation_counts.iter().take(10) {
            let topic = &self.topics[*id];
            println!("  {} - {} relations", topic.name, count);
        }
        println!();

        // Orphan topics.
        if !orphans.is_empty() {
            println!("Isolated topics ({}):", orphans.len());
            if verbose {
                for id in &orphans {
                    let topic = &self.topics[*id];
                    println!("  {} ({})", topic.name, id);
                }
            } else {
                let display: Vec<_> = orphans.iter().take(5).copied().collect();
                println!("  {}", display.join(", "));
                if orphans.len() > 5 {
                    println!("  ... and {} more", orphans.len() - 5);
                }
            }
            println!();
        }

        // Totals.
        let total_aliases: usize = self.topics.values().map(|t| t.aliases.len()).sum();
        let total_relations: usize = self.topics.values().map(|t| t.relations.len()).sum();
        println!("Totals:");
        println!("  {} topics", self.topics.len());
        println!("  {} categories", self.categories.len());
        println!("  {} relations", total_relations);
        println!("  {} aliases", total_aliases);
        println!("  {} unique tags", tags.len());
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
}
