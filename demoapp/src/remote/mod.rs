//! Remote content fetching with reqwest.

use rustmax::prelude::*;
use rustmax::reqwest;
use rustmax::log::info;
use std::path::Path;

use crate::{Error, Result};

/// Fetch content from a URL.
pub async fn fetch_content(url: &str) -> Result<String> {
    info!("Fetching content from {}", url);

    let response = reqwest::get(url)
        .await
        .map_err(|e| Error::remote(url, e.to_string()))?;

    if !response.status().is_success() {
        return Err(Error::remote(
            url,
            format!("HTTP {}", response.status()),
        ));
    }

    let content = response
        .text()
        .await
        .map_err(|e| Error::remote(url, e.to_string()))?;

    Ok(content)
}

/// Fetch a remote document and save it locally.
pub async fn fetch_document(url: &str, dest: &Path) -> Result<()> {
    let content = fetch_content(url).await?;

    // Ensure parent directory exists.
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(dest, &content)?;
    info!("Saved remote document to {}", dest.display());

    Ok(())
}

/// Fetch a binary asset from a URL.
pub async fn fetch_asset(url: &str) -> Result<Vec<u8>> {
    info!("Fetching asset from {}", url);

    let response = reqwest::get(url)
        .await
        .map_err(|e| Error::remote(url, e.to_string()))?;

    if !response.status().is_success() {
        return Err(Error::remote(
            url,
            format!("HTTP {}", response.status()),
        ));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| Error::remote(url, e.to_string()))?;

    Ok(bytes.to_vec())
}

/// Fetch multiple URLs in parallel.
pub async fn fetch_all(urls: &[&str]) -> Vec<Result<String>> {
    use rustmax::futures::future::join_all;

    let fetches = urls.iter().map(|url| fetch_content(url));
    join_all(fetches).await
}

/// Remote source configuration for a collection.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RemoteSource {
    /// Base URL for the remote source.
    pub url: String,
    /// Local path to sync to.
    pub path: String,
    /// File patterns to fetch (glob patterns).
    #[serde(default)]
    pub patterns: Vec<String>,
}

/// Fetch content from a remote source and save locally.
pub async fn sync_remote_source(source: &RemoteSource, collection_root: &Path) -> Result<usize> {
    info!("Syncing remote source: {}", source.url);

    // For now, just fetch the single URL and save it.
    // A more complete implementation would fetch an index file
    // listing all available documents.
    let dest = collection_root.join(&source.path);
    fetch_document(&source.url, &dest).await?;

    Ok(1)
}

/// Check if a URL is reachable.
pub async fn check_url(url: &str) -> Result<bool> {
    let client = reqwest::Client::new();
    let response = client
        .head(url)
        .send()
        .await
        .map_err(|e| Error::remote(url, e.to_string()))?;

    Ok(response.status().is_success())
}

/// Parse content type from URL for appropriate handling.
pub fn content_type_from_url(url: &str) -> ContentType {
    let lower = url.to_lowercase();
    if lower.ends_with(".md") || lower.ends_with(".markdown") {
        ContentType::Markdown
    } else if lower.ends_with(".html") || lower.ends_with(".htm") {
        ContentType::Html
    } else if lower.ends_with(".json") {
        ContentType::Json
    } else if lower.ends_with(".toml") {
        ContentType::Toml
    } else {
        ContentType::Unknown
    }
}

/// Content type for fetched content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Markdown,
    Html,
    Json,
    Toml,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_from_url() {
        assert_eq!(
            content_type_from_url("https://example.com/doc.md"),
            ContentType::Markdown
        );
        assert_eq!(
            content_type_from_url("https://example.com/page.html"),
            ContentType::Html
        );
        assert_eq!(
            content_type_from_url("https://example.com/data.json"),
            ContentType::Json
        );
        assert_eq!(
            content_type_from_url("https://example.com/config.toml"),
            ContentType::Toml
        );
        assert_eq!(
            content_type_from_url("https://example.com/file.xyz"),
            ContentType::Unknown
        );
    }
}
