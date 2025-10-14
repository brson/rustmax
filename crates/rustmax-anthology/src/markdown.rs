//! HTML to markdown conversion.

use rmx::prelude::*;
use rmx::std::path::Path;
use crate::metadata::Post;

/// Convert extracted HTML to markdown.
pub fn to_markdown(post: &Post, fetched_dir: &Path) -> AnyResult<()> {
    info!("Converting post to markdown: {}", post.id);

    let post_dir = fetched_dir.join(&post.id);

    // Read the extracted HTML.
    let extracted_path = post_dir.join("extracted.html");
    let html = rmx::std::fs::read_to_string(&extracted_path)
        .context("Failed to read extracted HTML")?;

    // Convert to markdown.
    let markdown = html2md::parse_html(&html);

    // Add frontmatter with metadata.
    let frontmatter = format!(
        "# {}\n\n**Author:** {}\n\n**Original URL:** <{}>\n\n---\n\n",
        post.title,
        post.author,
        post.url
    );

    let full_content = format!("{}{}", frontmatter, markdown);

    // Save the markdown.
    let markdown_path = post_dir.join("content.md");
    rmx::std::fs::write(&markdown_path, &full_content)
        .context("Failed to write markdown")?;

    info!("Successfully converted {} to {}", post.id, markdown_path.display());

    Ok(())
}

/// Check if a post has been converted to markdown.
pub fn has_markdown(post: &Post, fetched_dir: &Path) -> bool {
    let post_dir = fetched_dir.join(&post.id);
    let markdown_path = post_dir.join("content.md");
    markdown_path.exists()
}
