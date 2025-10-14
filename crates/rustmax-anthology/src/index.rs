//! Generate HTML index for the fetched directory.

use rmx::prelude::*;
use rmx::std::path::{Path, PathBuf};
use crate::metadata::{Post, PostCollection};

/// Generate an HTML index for the fetched directory.
pub fn generate_index(
    collection: &PostCollection,
    fetched_dir: &Path,
) -> AnyResult<()> {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("  <title>Anthology Fetched Content Index</title>\n");
    html.push_str("  <style>\n");
    html.push_str("    body { font-family: monospace; max-width: 1400px; margin: 0.5em; padding: 0; font-size: 0.85em; }\n");
    html.push_str("    h1 { font-size: 1.2em; margin: 0.5em 0; border-bottom: 1px solid #333; padding-bottom: 0.2em; }\n");
    html.push_str("    h2 { font-size: 1em; margin: 0.8em 0 0.3em 0; font-weight: bold; }\n");
    html.push_str("    .summary { background: #f0f0f0; padding: 0.3em 0.5em; margin-bottom: 0.5em; font-size: 0.9em; }\n");
    html.push_str("    .post { margin: 0.2em 0; padding: 0.3em 0.5em; border-left: 2px solid #ddd; cursor: pointer; }\n");
    html.push_str("    .post:hover { background: #f9f9f9; }\n");
    html.push_str("    .post-line { display: flex; gap: 1em; align-items: baseline; flex-wrap: wrap; }\n");
    html.push_str("    .post-title { font-weight: bold; }\n");
    html.push_str("    .post-meta { color: #666; font-size: 0.9em; }\n");
    html.push_str("    .artifacts { display: flex; gap: 0.5em; margin-left: auto; }\n");
    html.push_str("    .artifact { padding: 0.1em 0.4em; background: #e8e8e8; text-decoration: none; color: #0066cc; font-size: 0.85em; }\n");
    html.push_str("    .artifact:hover { background: #d0d0d0; }\n");
    html.push_str("    .artifact.missing { opacity: 0.3; pointer-events: none; color: #999; }\n");
    html.push_str("    a { color: #0066cc; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("  <h1>Anthology Fetched Content Index</h1>\n");

    // Summary
    let total = collection.posts.len();
    let mut with_raw = 0;
    let mut with_extracted = 0;
    let mut with_markdown = 0;

    for post in &collection.posts {
        let post_dir = fetched_dir.join(&post.id);
        if post_dir.join("raw.html").exists() { with_raw += 1; }
        if post_dir.join("extracted.html").exists() { with_extracted += 1; }
        if post_dir.join("content.md").exists() { with_markdown += 1; }
    }

    html.push_str("  <div class=\"summary\">\n");
    html.push_str(&format!("    Total: {} | Raw: {} | Extracted: {} | Markdown: {}\n",
        total, with_raw, with_extracted, with_markdown));
    html.push_str("  </div>\n");

    // Sort posts by category, then by ID
    let mut posts = collection.posts.clone();
    posts.sort_by(|a, b| {
        let cat_cmp = a.category.cmp(&b.category);
        if cat_cmp == std::cmp::Ordering::Equal {
            a.id.cmp(&b.id)
        } else {
            cat_cmp
        }
    });

    let mut current_category: Option<String> = None;

    for post in &posts {
        // Category header
        if current_category.as_ref() != Some(&post.category.clone().unwrap_or_else(|| "Uncategorized".to_string())) {
            let category = post.category.as_deref().unwrap_or("Uncategorized");
            if current_category.is_some() {
                html.push_str("  </div>\n"); // Close previous category
            }
            current_category = Some(category.to_string());
            html.push_str(&format!("  <h2>{}</h2>\n", category));
            html.push_str("  <div class=\"category\">\n");
        }

        let post_dir = fetched_dir.join(&post.id);

        // Check which artifacts exist
        let has_raw = post_dir.join("raw.html").exists();
        let has_extracted = post_dir.join("extracted.html").exists();
        let has_markdown = post_dir.join("content.md").exists();
        let has_fetch_info = post_dir.join("fetch-info.toml").exists();

        html.push_str(&format!("    <div class=\"post\" data-post-id=\"{}\">\n", post.id));
        html.push_str("      <div class=\"post-line\">\n");
        html.push_str(&format!("        <span class=\"post-title\">{}</span>\n", post.title));
        html.push_str(&format!("        <span class=\"post-meta\">by {} | <code>{}</code> | <a href=\"{}\">orig</a></span>\n",
            post.author, post.id, post.url));
        html.push_str("        <div class=\"artifacts\">\n");

        let missing = if !has_raw { " missing" } else { "" };
        html.push_str(&format!("          <a href=\"{}/raw.html\" class=\"artifact{}\">raw</a>\n",
            post.id, missing));

        let missing = if !has_extracted { " missing" } else { "" };
        html.push_str(&format!("          <a href=\"{}/extracted.html\" class=\"artifact{}\">ext</a>\n",
            post.id, missing));

        let missing = if !has_markdown { " missing" } else { "" };
        html.push_str(&format!("          <a href=\"{}/content.md\" class=\"artifact{}\">md</a>\n",
            post.id, missing));

        let missing = if !has_fetch_info { " missing" } else { "" };
        html.push_str(&format!("          <a href=\"{}/fetch-info.toml\" class=\"artifact{}\">info</a>\n",
            post.id, missing));

        html.push_str("        </div>\n");
        html.push_str("      </div>\n");
        html.push_str("    </div>\n");
    }

    if current_category.is_some() {
        html.push_str("  </div>\n"); // Close last category
    }

    html.push_str("  <script>\n");
    html.push_str("    document.querySelectorAll('.post').forEach(function(post) {\n");
    html.push_str("      post.addEventListener('click', function(e) {\n");
    html.push_str("        if (e.target.tagName === 'A') return;\n");
    html.push_str("        var postId = this.getAttribute('data-post-id');\n");
    html.push_str("        window.location.href = postId + '/content.md';\n");
    html.push_str("      });\n");
    html.push_str("    });\n");
    html.push_str("  </script>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    let index_path = fetched_dir.join("index.html");
    rmx::std::fs::write(&index_path, html)
        .context("Failed to write index.html")?;

    info!("Generated index at: {}", index_path.display());

    Ok(())
}
