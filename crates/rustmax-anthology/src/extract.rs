//! Content extraction coordinator.

use rmx::prelude::*;
use rmx::std::path::Path;
use crate::metadata::Post;
use crate::extractors;

/// Extract content from a fetched post.
pub fn extract_post(post: &Post, fetched_dir: &Path) -> AnyResult<()> {
    info!("Extracting content for post: {}", post.id);

    let post_dir = fetched_dir.join(&post.id);

    // Read the raw HTML.
    let raw_path = post_dir.join("raw.html");
    let html = rmx::std::fs::read_to_string(&raw_path)
        .context("Failed to read raw HTML")?;

    // Get the appropriate extractor.
    let extractor = extractors::get_extractor(&post.extractor)
        .context("Failed to get extractor")?;

    // Extract the content.
    let extracted = extractor.extract(&html)
        .context("Failed to extract content")?;

    // Save the extracted content.
    let extracted_path = post_dir.join("extracted.html");
    rmx::std::fs::write(&extracted_path, &extracted)
        .context("Failed to write extracted HTML")?;

    info!("Successfully extracted {} to {}", post.id, extracted_path.display());

    Ok(())
}

/// Check if a post has been extracted.
pub fn is_extracted(post: &Post, fetched_dir: &Path) -> bool {
    let post_dir = fetched_dir.join(&post.id);
    let extracted_path = post_dir.join("extracted.html");
    extracted_path.exists()
}
