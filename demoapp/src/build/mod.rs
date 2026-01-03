//! Build pipeline for generating static output.

mod markdown;
mod template;
mod compress;
mod rewrite;
mod encoding;

pub use markdown::render_markdown;
pub use template::TemplateEngine;
pub use compress::{compress_output, compress, decompress, compress_with_level, CompressStats};
pub use rewrite::{
    UrlRewriter, make_urls_absolute, rewrite_md_links, extract_urls,
    slugify, is_valid_slug, replace_pattern, find_internal_links, verify_links
};
pub use encoding::{
    to_base64, from_base64, to_hex, from_hex, create_data_url, file_to_data_url,
    guess_mime_type, AssetBuffer, format_hash_short, format_size
};

use rustmax::prelude::*;
use rustmax::rayon::prelude::*;
use rustmax::log::{info, debug};
use rustmax::jiff::Zoned;
use std::path::Path;
use std::fs;

use crate::collection::{Collection, Config, Document};
use crate::Result;

/// Build the collection to static output.
pub fn build(
    collection: &Collection,
    config: &Config,
    output_dir: &Path,
    include_drafts: bool,
) -> Result<()> {
    // Clean and create output directory.
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    // Initialize template engine.
    let templates_dir = collection.root.join("templates");
    let engine = TemplateEngine::new(&templates_dir)?;

    // Filter documents.
    let documents: Vec<&Document> = if include_drafts {
        collection.all_sorted()
    } else {
        collection.published()
    };

    info!("Building {} documents", documents.len());

    // Build documents in parallel.
    let results: Vec<Result<()>> = documents
        .par_iter()
        .map(|doc| build_document(doc, config, &engine, output_dir))
        .collect();

    // Check for errors.
    for result in results {
        result?;
    }

    // Build index page.
    build_index(&documents, config, &engine, output_dir)?;

    // Build tag pages.
    build_tag_pages(collection, config, &engine, output_dir, include_drafts)?;

    // Copy static assets.
    let static_dir = collection.root.join("static");
    if static_dir.exists() {
        copy_static(&static_dir, output_dir)?;
    }

    Ok(())
}

/// Build a single document.
fn build_document(
    doc: &Document,
    config: &Config,
    engine: &TemplateEngine,
    output_dir: &Path,
) -> Result<()> {
    debug!("Building: {}", doc.source_path.display());

    let html_content = render_markdown(&doc.content);

    let template_name = doc
        .frontmatter
        .template
        .as_deref()
        .unwrap_or(&config.content.default_template);

    let context = engine.document_context(doc, config, &html_content);
    let rendered = engine.render(template_name, &context)?;

    // Write to output.
    let doc_dir = output_dir.join(doc.slug());
    fs::create_dir_all(&doc_dir)?;
    fs::write(doc_dir.join("index.html"), rendered)?;

    Ok(())
}

/// Build the index page.
fn build_index(
    documents: &[&Document],
    config: &Config,
    engine: &TemplateEngine,
    output_dir: &Path,
) -> Result<()> {
    let context = engine.index_context(documents, config);
    let rendered = engine.render("index.html", &context).or_else(|_| {
        // Fall back to default template with listing.
        engine.render("default.html", &context)
    })?;

    fs::write(output_dir.join("index.html"), rendered)?;
    Ok(())
}

/// Build tag index pages.
fn build_tag_pages(
    collection: &Collection,
    config: &Config,
    engine: &TemplateEngine,
    output_dir: &Path,
    include_drafts: bool,
) -> Result<()> {
    let tags_dir = output_dir.join("tags");
    fs::create_dir_all(&tags_dir)?;

    for tag in collection.tags() {
        let documents: Vec<&Document> = collection
            .by_tag(&tag)
            .into_iter()
            .filter(|d| include_drafts || !d.frontmatter.draft)
            .collect();

        if documents.is_empty() {
            continue;
        }

        let context = engine.tag_context(&tag, &documents, config);
        let rendered = engine.render("tag.html", &context).or_else(|_| {
            engine.render("default.html", &context)
        })?;

        let tag_dir = tags_dir.join(&tag);
        fs::create_dir_all(&tag_dir)?;
        fs::write(tag_dir.join("index.html"), rendered)?;
    }

    Ok(())
}

/// Copy static assets to output.
fn copy_static(static_dir: &Path, output_dir: &Path) -> Result<()> {
    use rustmax::walkdir::WalkDir;

    for entry in WalkDir::new(static_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let relative = path.strip_prefix(static_dir).unwrap();
            let dest = output_dir.join(relative);

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, dest)?;
        }
    }

    Ok(())
}

/// Generate RSS feed.
pub fn generate_rss(collection: &Collection, config: &Config) -> Result<String> {
    let documents = collection.published();
    let now = Zoned::now();

    let mut items = String::new();
    for doc in documents.iter().take(20) {
        let date = doc
            .frontmatter
            .date
            .map(|d| d.to_string())
            .unwrap_or_default();

        let excerpt = doc.excerpt(&config.content.excerpt_separator, 300);

        items.push_str(&format!(
            r#"    <item>
      <title>{}</title>
      <link>{}{}</link>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
    </item>
"#,
            html_escape(&doc.frontmatter.title),
            config.collection.base_url,
            doc.url_path(),
            date,
            excerpt
        ));
    }

    let rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <lastBuildDate>{}</lastBuildDate>
{}  </channel>
</rss>
"#,
        html_escape(&config.collection.title),
        config.collection.base_url,
        html_escape(&config.collection.description),
        now.strftime("%a, %d %b %Y %H:%M:%S %z"),
        items
    );

    Ok(rss)
}

/// Generate sitemap.
pub fn generate_sitemap(collection: &Collection, config: &Config) -> Result<String> {
    let documents = collection.published();

    let mut urls = String::new();
    for doc in &documents {
        urls.push_str(&format!(
            r#"  <url>
    <loc>{}{}</loc>
  </url>
"#,
            config.collection.base_url,
            doc.url_path()
        ));
    }

    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>
"#,
        urls
    );

    Ok(sitemap)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
