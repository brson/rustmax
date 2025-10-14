//! Build the mdbook from processed posts.

use rmx::prelude::*;
use rmx::std::collections::BTreeMap;
use rmx::std::path::{Path, PathBuf};
use crate::metadata::{Post, PostCollection};

/// Check if a post is ready for inclusion in the book.
pub fn is_post_ready(post: &Post, fetched_dir: &Path) -> bool {
    let post_dir = fetched_dir.join(&post.id);
    let markdown_path = post_dir.join("content.md");
    markdown_path.exists()
}

/// Build the mdbook from the processed posts.
pub fn build_book(
    collection: &PostCollection,
    fetched_dir: &Path,
    book_dir: &Path,
) -> AnyResult<()> {
    let src_dir = book_dir.join("src");

    // Generate SUMMARY.md
    generate_summary(collection, fetched_dir, &src_dir)?;

    // Copy markdown files to book/src/
    copy_post_content(collection, fetched_dir, &src_dir)?;

    // Run mdbook build
    info!("Building mdbook...");
    let output = std::process::Command::new("mdbook")
        .arg("build")
        .current_dir(book_dir)
        .output()
        .context("Failed to run mdbook build")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("mdbook build failed: {}", stderr);
    }

    let build_dir = book_dir.join("rendered");
    info!("Book built successfully at: {}", build_dir.display());
    info!("Open {}/index.html to view", build_dir.display());

    Ok(())
}

/// Generate SUMMARY.md from posts metadata.
fn generate_summary(
    collection: &PostCollection,
    fetched_dir: &Path,
    src_dir: &Path,
) -> AnyResult<()> {
    let mut summary = String::new();
    summary.push_str("# Summary\n\n");
    summary.push_str("[Introduction](README.md)\n\n");

    // Group posts by category
    let mut categories: BTreeMap<String, Vec<&Post>> = BTreeMap::new();
    for post in &collection.posts {
        if !is_post_ready(post, fetched_dir) {
            warn!("Skipping post (not ready): {}", post.id);
            continue;
        }

        let category = post.category.as_deref().unwrap_or("Uncategorized");
        categories.entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(post);
    }

    // Write categories and posts
    for (category, posts) in categories {
        summary.push_str(&format!("# {}\n\n", category));
        for post in posts {
            let filename = format!("{}.md", post.id);
            summary.push_str(&format!("- [{}]({})\n", post.title, filename));
        }
        summary.push('\n');
    }

    let summary_path = src_dir.join("SUMMARY.md");
    rmx::std::fs::write(&summary_path, summary)
        .context("Failed to write SUMMARY.md")?;

    info!("Generated SUMMARY.md");
    Ok(())
}

/// Copy post content markdown files to book/src/.
fn copy_post_content(
    collection: &PostCollection,
    fetched_dir: &Path,
    src_dir: &Path,
) -> AnyResult<()> {
    for post in &collection.posts {
        if !is_post_ready(post, fetched_dir) {
            continue;
        }

        let source_path = fetched_dir.join(&post.id).join("content.md");
        let dest_filename = format!("{}.md", post.id);
        let dest_path = src_dir.join(&dest_filename);

        // Read the content and add metadata header
        let content = rmx::std::fs::read_to_string(&source_path)
            .context(format!("Failed to read {}", source_path.display()))?;

        let mut full_content = String::new();
        full_content.push_str(&format!("# {}\n\n", post.title));
        full_content.push_str(&format!("**Author:** {}\n\n", post.author));
        full_content.push_str(&format!("**Original:** [{}]({})\n\n", post.url, post.url));
        full_content.push_str("---\n\n");
        full_content.push_str(&content);

        rmx::std::fs::write(&dest_path, full_content)
            .context(format!("Failed to write {}", dest_path.display()))?;

        debug!("Copied content for: {}", post.id);
    }

    info!("Copied markdown files to book");
    Ok(())
}
