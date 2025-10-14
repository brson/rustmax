//! Metadata structures for blog posts.

use rmx::prelude::*;
use rmx::std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/// Metadata for a single blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Unique identifier (slug).
    pub id: String,

    /// Post title.
    pub title: String,

    /// Author name.
    pub author: String,

    /// Original URL.
    pub url: String,

    /// Original publication date (if known).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,

    /// Category or section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Which extractor to use (default or custom site name).
    #[serde(default = "default_extractor")]
    pub extractor: String,

    /// Date we fetched this post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched_date: Option<String>,

    /// Optional notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

fn default_extractor() -> String {
    "default".to_string()
}

/// Collection of all posts.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostCollection {
    pub posts: Vec<Post>,
}

impl PostCollection {
    /// Load posts from TOML file.
    pub fn load(path: &Path) -> AnyResult<Self> {
        let content = rmx::std::fs::read_to_string(path)
            .context("Failed to read posts.toml")?;
        let collection = rmx::toml::from_str(&content)
            .context("Failed to parse posts.toml")?;
        Ok(collection)
    }

    /// Save posts to TOML file.
    pub fn save(&self, path: &Path) -> AnyResult<()> {
        let content = rmx::toml::to_string_pretty(self)
            .context("Failed to serialize posts")?;
        rmx::std::fs::write(path, content)
            .context("Failed to write posts.toml")?;
        Ok(())
    }

    /// Find a post by ID.
    pub fn find(&self, id: &str) -> Option<&Post> {
        self.posts.iter().find(|p| p.id == id)
    }

    /// Find a post by ID (mutable).
    pub fn find_mut(&mut self, id: &str) -> Option<&mut Post> {
        self.posts.iter_mut().find(|p| p.id == id)
    }
}

/// Information about a fetch operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchInfo {
    /// URL that was fetched.
    pub url: String,

    /// Timestamp of fetch.
    pub fetched_at: String,

    /// HTTP status code.
    pub status_code: u16,

    /// Final URL after redirects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,

    /// Content-Type header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl FetchInfo {
    /// Save fetch info to TOML file.
    pub fn save(&self, path: &Path) -> AnyResult<()> {
        let content = rmx::toml::to_string_pretty(self)
            .context("Failed to serialize fetch info")?;
        rmx::std::fs::write(path, content)
            .context("Failed to write fetch info")?;
        Ok(())
    }
}
