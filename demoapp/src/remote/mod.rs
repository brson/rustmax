//! `folio fetch`: download markdown documents into a collection.
//!
//! URLs are checked with `url` before anything touches the network,
//! responses are checked with `mime` before anything touches the disk,
//! and several URLs are fetched concurrently with `futures`.

use rmx::futures::future::join_all;
use rmx::log::info;
use rmx::mime::{self, Mime};
use rmx::prelude::*;
use rmx::reqwest::{self, header::CONTENT_TYPE};
use rmx::url::Url;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// Parse a URL to fetch, which has to be `http` or `https`.
pub fn parse_url(input: &str) -> Result<Url> {
    let url = Url::parse(input).map_err(|e| Error::remote(input, format!("not a URL: {e}")))?;
    match url.scheme() {
        "http" | "https" => Ok(url),
        scheme => Err(Error::remote(input, format!("cannot fetch `{scheme}` URLs"))),
    }
}

/// The file name a fetched document is saved under.
///
/// The last segment of the URL's path, given a `.md` extension unless it
/// already has a markdown one; `index.md` if the path has no segments.
pub fn file_name(url: &Url) -> String {
    let segment = url
        .path_segments()
        .and_then(|mut segments| segments.rfind(|s| !s.is_empty()))
        .unwrap_or("index");

    let is_markdown = Path::new(segment)
        .extension()
        .is_some_and(|ext| ext == "md" || ext == "markdown");

    if is_markdown {
        segment.to_string()
    } else {
        format!("{segment}.md")
    }
}

/// Check that a response's `Content-Type` could be a markdown document.
///
/// Markdown is served as `text/markdown`, as `text/plain` by most raw-file
/// hosts, or with no type at all. An HTML page is the usual mistake: the URL
/// of a rendered page rather than of its source.
pub fn check_content_type(content_type: Option<&str>) -> std::result::Result<(), String> {
    let Some(content_type) = content_type else {
        return Ok(());
    };

    let mime: Mime = content_type
        .parse()
        .map_err(|_| format!("unparseable Content-Type `{content_type}`"))?;

    if mime.type_() != mime::TEXT {
        return Err(format!("served as `{}`, not text", mime.essence_str()));
    }
    if mime.subtype() == mime::HTML {
        return Err("served as an HTML page; fetch the raw markdown instead".to_string());
    }
    Ok(())
}

/// Download one document's text.
pub async fn fetch_text(client: &reqwest::Client, url: &Url) -> Result<String> {
    let remote = |message: String| Error::remote(url.as_str(), message);

    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| remote(e.to_string()))?;

    if !response.status().is_success() {
        return Err(remote(format!("HTTP {}", response.status())));
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .map(|value| value.to_str().map_err(|_| remote("non-ASCII Content-Type".to_string())))
        .transpose()?;
    check_content_type(content_type).map_err(remote)?;

    response.text().await.map_err(|e| remote(e.to_string()))
}

/// Download a document to `dest`, which must not already exist.
pub async fn fetch_document(client: &reqwest::Client, url: &Url, dest: &Path) -> Result<()> {
    if dest.exists() {
        return Err(Error::document(dest, "already exists; not overwriting it"));
    }

    let text = fetch_text(client, url).await?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(dest, text)?;
    info!("Fetched {url} -> {}", dest.display());
    Ok(())
}

/// Download several documents concurrently.
///
/// Every download is attempted, and each one's outcome is returned in order,
/// so one bad URL does not lose the others.
pub async fn fetch_documents(downloads: &[(Url, PathBuf)]) -> Vec<Result<()>> {
    let client = reqwest::Client::new();
    join_all(
        downloads
            .iter()
            .map(|(url, dest)| fetch_document(&client, url, dest)),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_http_urls_are_fetched() {
        assert!(parse_url("https://example.com/a.md").is_ok());
        assert!(parse_url("http://example.com/a.md").is_ok());
        assert!(parse_url("file:///etc/passwd").unwrap_err().to_string().contains("`file`"));
        assert!(parse_url("not a url").unwrap_err().to_string().contains("not a URL"));
    }

    #[test]
    fn file_names_come_from_the_last_path_segment() {
        let name = |s: &str| file_name(&Url::parse(s).unwrap());
        assert_eq!(name("https://example.com/docs/guide.md"), "guide.md");
        assert_eq!(name("https://example.com/notes.markdown"), "notes.markdown");
        assert_eq!(name("https://example.com/blog/post/"), "post.md");
        assert_eq!(name("https://example.com/readme.txt"), "readme.txt.md");
        assert_eq!(name("https://example.com/"), "index.md");
        assert_eq!(name("https://example.com/a.md?raw=true"), "a.md");
    }

    #[test]
    fn markdown_and_plain_text_are_accepted() {
        assert!(check_content_type(None).is_ok());
        assert!(check_content_type(Some("text/markdown; charset=utf-8")).is_ok());
        assert!(check_content_type(Some("text/plain")).is_ok());
        assert!(check_content_type(Some("Text/X-Markdown")).is_ok());
    }

    #[test]
    fn html_pages_and_binaries_are_rejected() {
        let err = check_content_type(Some("text/html; charset=utf-8")).unwrap_err();
        assert!(err.contains("HTML page"), "{err}");

        let err = check_content_type(Some("image/png")).unwrap_err();
        assert!(err.contains("image/png"), "{err}");

        assert!(check_content_type(Some("???")).is_err());
    }
}
