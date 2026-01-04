//! Remote content fetching with reqwest.

use rustmax::prelude::*;
use rustmax::reqwest;
use rustmax::url::Url;
use rustmax::mime::{self, Mime};
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

/// Parse and validate a URL string.
pub fn parse_url(url: &str) -> Result<Url> {
    Url::parse(url).map_err(|e| Error::remote(url, format!("invalid URL: {}", e)))
}

/// Extract the filename from a URL path.
pub fn filename_from_url(url: &Url) -> Option<String> {
    url.path_segments()
        .and_then(|segments| segments.last())
        .filter(|s| !s.is_empty() && s.contains('.'))
        .map(|s| s.to_string())
}

/// Get the domain from a URL.
pub fn domain_from_url(url: &Url) -> Option<String> {
    url.host_str().map(|s| s.to_string())
}

/// Guess MIME type from a file extension.
pub fn mime_from_extension(ext: &str) -> Mime {
    match ext.to_lowercase().as_str() {
        "html" | "htm" => mime::TEXT_HTML,
        "css" => mime::TEXT_CSS,
        "js" => mime::APPLICATION_JAVASCRIPT,
        "json" => mime::APPLICATION_JSON,
        "xml" => mime::TEXT_XML,
        "txt" | "md" | "markdown" => mime::TEXT_PLAIN,
        "png" => mime::IMAGE_PNG,
        "jpg" | "jpeg" => mime::IMAGE_JPEG,
        "gif" => mime::IMAGE_GIF,
        "svg" => mime::IMAGE_SVG,
        "webp" => "image/webp".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM),
        "pdf" => mime::APPLICATION_PDF,
        "woff" => "font/woff".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM),
        "woff2" => "font/woff2".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM),
        _ => mime::APPLICATION_OCTET_STREAM,
    }
}

/// Guess MIME type from a URL.
pub fn mime_from_url(url: &Url) -> Mime {
    let path = url.path();
    if let Some(ext) = path.rsplit('.').next() {
        mime_from_extension(ext)
    } else {
        mime::APPLICATION_OCTET_STREAM
    }
}

/// Parse content type from URL for appropriate handling.
pub fn content_type_from_url(url: &str) -> ContentType {
    if let Ok(parsed) = Url::parse(url) {
        let mime = mime_from_url(&parsed);
        if mime == mime::TEXT_HTML {
            ContentType::Html
        } else if mime == mime::APPLICATION_JSON {
            ContentType::Json
        } else if mime.type_() == mime::TEXT && parsed.path().ends_with(".md") {
            ContentType::Markdown
        } else if parsed.path().ends_with(".toml") {
            ContentType::Toml
        } else {
            ContentType::Unknown
        }
    } else {
        // Fall back to extension-based detection for invalid URLs.
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
    fn test_parse_url() {
        let url = parse_url("https://example.com/path/to/file.md").unwrap();
        assert_eq!(url.host_str(), Some("example.com"));
        assert_eq!(url.path(), "/path/to/file.md");
    }

    #[test]
    fn test_parse_url_invalid() {
        assert!(parse_url("not a url").is_err());
    }

    #[test]
    fn test_filename_from_url() {
        let url = Url::parse("https://example.com/path/file.md").unwrap();
        assert_eq!(filename_from_url(&url), Some("file.md".to_string()));

        let url = Url::parse("https://example.com/path/").unwrap();
        assert_eq!(filename_from_url(&url), None);

        let url = Url::parse("https://example.com/").unwrap();
        assert_eq!(filename_from_url(&url), None);
    }

    #[test]
    fn test_domain_from_url() {
        let url = Url::parse("https://example.com/path").unwrap();
        assert_eq!(domain_from_url(&url), Some("example.com".to_string()));

        let url = Url::parse("https://sub.example.com:8080/path").unwrap();
        assert_eq!(domain_from_url(&url), Some("sub.example.com".to_string()));
    }

    #[test]
    fn test_mime_from_extension() {
        assert_eq!(mime_from_extension("html"), mime::TEXT_HTML);
        assert_eq!(mime_from_extension("css"), mime::TEXT_CSS);
        assert_eq!(mime_from_extension("js"), mime::APPLICATION_JAVASCRIPT);
        assert_eq!(mime_from_extension("json"), mime::APPLICATION_JSON);
        assert_eq!(mime_from_extension("png"), mime::IMAGE_PNG);
        assert_eq!(mime_from_extension("jpg"), mime::IMAGE_JPEG);
        assert_eq!(mime_from_extension("gif"), mime::IMAGE_GIF);
        assert_eq!(mime_from_extension("svg"), mime::IMAGE_SVG);
        assert_eq!(mime_from_extension("pdf"), mime::APPLICATION_PDF);
        assert_eq!(mime_from_extension("unknown"), mime::APPLICATION_OCTET_STREAM);
    }

    #[test]
    fn test_mime_from_url() {
        let url = Url::parse("https://example.com/style.css").unwrap();
        assert_eq!(mime_from_url(&url), mime::TEXT_CSS);

        let url = Url::parse("https://example.com/image.png").unwrap();
        assert_eq!(mime_from_url(&url), mime::IMAGE_PNG);

        let url = Url::parse("https://example.com/noext").unwrap();
        assert_eq!(mime_from_url(&url), mime::APPLICATION_OCTET_STREAM);
    }

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
