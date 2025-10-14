//! HTTP fetching infrastructure.

use rmx::prelude::*;
use rmx::std::path::{Path, PathBuf};
use crate::metadata::{FetchInfo, Post};

/// Fetch a blog post from its URL.
pub fn fetch_post(post: &Post, output_dir: &Path) -> AnyResult<()> {
    info!("Fetching post: {} from {}", post.id, post.url);

    // Create output directory for this post.
    let post_dir = output_dir.join(&post.id);
    rmx::std::fs::create_dir_all(&post_dir)
        .context("Failed to create post directory")?;

    // Fetch the URL.
    let response = rmx::reqwest::blocking::get(&post.url)
        .context("Failed to fetch URL")?;

    let status_code = response.status().as_u16();
    let final_url = response.url().to_string();
    let content_type = response.headers()
        .get(rmx::reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Get the HTML content.
    let html = response.text()
        .context("Failed to read response body")?;

    // Save the raw HTML.
    let raw_path = post_dir.join("raw.html");
    rmx::std::fs::write(&raw_path, &html)
        .context("Failed to write raw HTML")?;

    // Save fetch info.
    let fetch_info = FetchInfo {
        url: post.url.clone(),
        fetched_at: rmx::jiff::Zoned::now().to_string(),
        status_code,
        final_url: if final_url != post.url {
            Some(final_url)
        } else {
            None
        },
        content_type,
    };

    let info_path = post_dir.join("fetch-info.toml");
    fetch_info.save(&info_path)?;

    info!("Successfully fetched {} to {}", post.id, post_dir.display());

    Ok(())
}

/// Check if a post has been fetched.
pub fn is_fetched(post: &Post, output_dir: &Path) -> bool {
    let post_dir = output_dir.join(&post.id);
    let raw_path = post_dir.join("raw.html");
    raw_path.exists()
}
