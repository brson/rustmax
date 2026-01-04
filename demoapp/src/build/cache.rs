//! Build cache for incremental builds.
//!
//! Tracks content hashes to skip rebuilding unchanged documents.

use rustmax::prelude::*;
use rustmax::log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::Result;

/// Cache file name.
const CACHE_FILE: &str = ".anthology-cache.json";

/// Build cache for tracking document changes.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildCache {
    /// Map of source path -> cached entry.
    entries: HashMap<PathBuf, CacheEntry>,
    /// Template directory hash (if templates change, rebuild all).
    template_hash: Option<String>,
}

/// Cache entry for a single document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Content hash of the source file.
    pub content_hash: String,
    /// Output path that was generated.
    pub output_path: PathBuf,
    /// Last build timestamp (Unix seconds).
    pub built_at: u64,
}

/// Result of checking if a document needs rebuilding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheStatus {
    /// Document has not changed, skip rebuild.
    Fresh,
    /// Document is new or changed, needs rebuild.
    Stale,
    /// Cache is invalid (templates changed, etc.).
    Invalid,
}

impl BuildCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load cache from the collection root.
    pub fn load(root: &Path) -> Self {
        let cache_path = root.join(CACHE_FILE);

        if !cache_path.exists() {
            debug!("No build cache found, starting fresh");
            return Self::new();
        }

        match std::fs::read_to_string(&cache_path) {
            Ok(json) => match rustmax::serde_json::from_str(&json) {
                Ok(cache) => {
                    debug!("Loaded build cache with {} entries", Self::entry_count(&cache));
                    cache
                }
                Err(e) => {
                    debug!("Failed to parse build cache: {}", e);
                    Self::new()
                }
            },
            Err(e) => {
                debug!("Failed to read build cache: {}", e);
                Self::new()
            }
        }
    }

    fn entry_count(cache: &Self) -> usize {
        cache.entries.len()
    }

    /// Save cache to the collection root.
    pub fn save(&self, root: &Path) -> Result<()> {
        let cache_path = root.join(CACHE_FILE);
        let json = rustmax::serde_json::to_string_pretty(self)?;
        std::fs::write(&cache_path, json)?;
        debug!("Saved build cache with {} entries", self.entries.len());
        Ok(())
    }

    /// Check if a document needs rebuilding.
    pub fn check(&self, source_path: &Path, content_hash: &str, output_path: &Path) -> CacheStatus {
        match self.entries.get(source_path) {
            Some(entry) => {
                // Check if content hash matches.
                if entry.content_hash != content_hash {
                    debug!("Content changed: {}", source_path.display());
                    return CacheStatus::Stale;
                }

                // Check if output file exists.
                if !output_path.exists() {
                    debug!("Output missing: {}", output_path.display());
                    return CacheStatus::Stale;
                }

                CacheStatus::Fresh
            }
            None => {
                debug!("Not in cache: {}", source_path.display());
                CacheStatus::Stale
            }
        }
    }

    /// Update cache entry after successful build.
    pub fn update(&mut self, source_path: PathBuf, content_hash: String, output_path: PathBuf) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.entries.insert(
            source_path,
            CacheEntry {
                content_hash,
                output_path,
                built_at: now,
            },
        );
    }

    /// Remove entries for documents that no longer exist.
    pub fn prune(&mut self, existing_sources: &[PathBuf]) {
        let existing: std::collections::HashSet<_> = existing_sources.iter().collect();
        let before = self.entries.len();

        self.entries.retain(|path, _| existing.contains(path));

        let removed = before - self.entries.len();
        if removed > 0 {
            debug!("Pruned {} stale cache entries", removed);
        }
    }

    /// Get the template hash.
    pub fn template_hash(&self) -> Option<&str> {
        self.template_hash.as_deref()
    }

    /// Set the template hash.
    pub fn set_template_hash(&mut self, hash: String) {
        self.template_hash = Some(hash);
    }

    /// Check if templates have changed.
    pub fn templates_changed(&self, current_hash: &str) -> bool {
        match &self.template_hash {
            Some(cached) => cached != current_hash,
            None => true, // No cached hash means first build.
        }
    }

    /// Clear all entries (force full rebuild).
    pub fn clear(&mut self) {
        self.entries.clear();
        self.template_hash = None;
    }

    /// Get statistics about the cache.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.entries.len(),
            has_template_hash: self.template_hash.is_some(),
        }
    }

    /// Get entry for a source path.
    pub fn get(&self, source_path: &Path) -> Option<&CacheEntry> {
        self.entries.get(source_path)
    }
}

/// Statistics about the build cache.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub has_template_hash: bool,
}

/// Compute a hash for the templates directory.
pub fn hash_templates(templates_dir: &Path) -> Result<String> {
    use rustmax::blake3::Hasher;
    use rustmax::walkdir::WalkDir;

    let mut hasher = Hasher::new();

    if !templates_dir.exists() {
        return Ok("no-templates".to_string());
    }

    // Hash all template files in sorted order for determinism.
    let mut paths: Vec<_> = WalkDir::new(templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    paths.sort();

    for path in paths {
        if let Ok(content) = std::fs::read(&path) {
            hasher.update(&content);
        }
    }

    Ok(hasher.finalize().to_hex().to_string())
}

/// Build result with cache statistics.
#[derive(Debug, Clone)]
pub struct IncrementalBuildResult {
    /// Documents that were rebuilt.
    pub rebuilt: usize,
    /// Documents that were skipped (cache hit).
    pub skipped: usize,
    /// Total documents.
    pub total: usize,
}

impl IncrementalBuildResult {
    /// Create a new result.
    pub fn new() -> Self {
        Self {
            rebuilt: 0,
            skipped: 0,
            total: 0,
        }
    }

    /// Log the build result.
    pub fn log(&self) {
        if self.skipped > 0 {
            info!(
                "Built {} documents ({} rebuilt, {} cached)",
                self.total, self.rebuilt, self.skipped
            );
        } else {
            info!("Built {} documents", self.total);
        }
    }
}

impl Default for IncrementalBuildResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax::tempfile::tempdir;

    #[test]
    fn test_cache_new() {
        let cache = BuildCache::new();
        assert_eq!(cache.entries.len(), 0);
        assert!(cache.template_hash.is_none());
    }

    #[test]
    fn test_cache_update_and_check() {
        let mut cache = BuildCache::new();
        let source = PathBuf::from("test.md");
        let output = PathBuf::from("output/test/index.html");

        // Initially stale.
        assert_eq!(cache.check(&source, "abc123", &output), CacheStatus::Stale);

        // Update cache.
        cache.update(source.clone(), "abc123".to_string(), output.clone());

        // Now fresh (but output doesn't exist, so still stale).
        assert_eq!(cache.check(&source, "abc123", &output), CacheStatus::Stale);

        // Different hash is stale.
        assert_eq!(cache.check(&source, "different", &output), CacheStatus::Stale);
    }

    #[test]
    fn test_cache_save_load() {
        let dir = tempdir().unwrap();
        let mut cache = BuildCache::new();

        cache.update(
            PathBuf::from("doc.md"),
            "hash123".to_string(),
            PathBuf::from("output/doc/index.html"),
        );
        cache.set_template_hash("template-hash".to_string());

        cache.save(dir.path()).unwrap();

        let loaded = BuildCache::load(dir.path());
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.template_hash(), Some("template-hash"));
    }

    #[test]
    fn test_cache_prune() {
        let mut cache = BuildCache::new();

        cache.update(
            PathBuf::from("keep.md"),
            "hash1".to_string(),
            PathBuf::from("out/keep"),
        );
        cache.update(
            PathBuf::from("remove.md"),
            "hash2".to_string(),
            PathBuf::from("out/remove"),
        );

        assert_eq!(cache.entries.len(), 2);

        cache.prune(&[PathBuf::from("keep.md")]);

        assert_eq!(cache.entries.len(), 1);
        assert!(cache.entries.contains_key(&PathBuf::from("keep.md")));
    }

    #[test]
    fn test_templates_changed() {
        let mut cache = BuildCache::new();

        // No hash means changed.
        assert!(cache.templates_changed("new-hash"));

        cache.set_template_hash("old-hash".to_string());

        // Same hash means not changed.
        assert!(!cache.templates_changed("old-hash"));

        // Different hash means changed.
        assert!(cache.templates_changed("new-hash"));
    }

    #[test]
    fn test_hash_templates() {
        let dir = tempdir().unwrap();
        let templates_dir = dir.path().join("templates");
        std::fs::create_dir(&templates_dir).unwrap();

        std::fs::write(templates_dir.join("default.html"), "<html></html>").unwrap();

        let hash1 = hash_templates(&templates_dir).unwrap();
        assert!(!hash1.is_empty());

        // Same content, same hash.
        let hash2 = hash_templates(&templates_dir).unwrap();
        assert_eq!(hash1, hash2);

        // Different content, different hash.
        std::fs::write(templates_dir.join("default.html"), "<html>changed</html>").unwrap();
        let hash3 = hash_templates(&templates_dir).unwrap();
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = BuildCache::new();
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 0);
        assert!(!stats.has_template_hash);

        cache.update(PathBuf::from("a.md"), "h".to_string(), PathBuf::from("o"));
        cache.set_template_hash("t".to_string());

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 1);
        assert!(stats.has_template_hash);
    }
}
