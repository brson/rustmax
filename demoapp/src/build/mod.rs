//! Build pipeline for generating static output.

mod markdown;
mod template;
mod compress;
mod cache;
mod highlight;
mod toc;
mod search_js;
mod progress;
mod assets;

pub use markdown::{render_markdown, render_markdown_highlighted, apply_syntax_highlighting, generate_highlight_css};
pub use template::TemplateEngine;
pub use compress::{compress_output, CompressStats};
pub use cache::{BuildCache, CacheStatus, IncrementalBuildResult, hash_templates};
pub use highlight::{
    Highlighter, Theme, ThemeColors, Token, TokenType,
    HighlightOptions, tokenize,
    themes, languages,
};
pub use toc::{
    TableOfContents, TocEntry, TocOptions, Heading, HeadingLevel,
    extract_headings_html, generate_id, generate_toc_css,
};
pub use search_js::{generate_search_js, generate_search_page};
pub use progress::{
    document_progress, incremental_progress,
    asset_spinner, template_spinner, search_spinner, feed_spinner, compress_spinner,
    finish_with_check,
};
pub use assets::{Asset, site_assets, integrity, integrity_map};

use rmx::prelude::*;
use rmx::rayon::prelude::*;
use rmx::indicatif::ProgressBar;
use rmx::log::{info, debug};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs;

use crate::collection::{Collection, Config, Document};
use crate::features::BuildFeatures;
use crate::site::SiteIndex;
use crate::Result;

/// Build the collection to static output.
///
/// What is built beyond the document pages is decided by
/// `config.build.features`.
pub fn build(collection: &Collection, config: &Config, output_dir: &Path) -> Result<()> {
    build_full(collection, config, output_dir, false)
}

/// Build the collection with progress bars.
pub fn build_with_progress(collection: &Collection, config: &Config, output_dir: &Path) -> Result<()> {
    build_full(collection, config, output_dir, true)
}

fn build_full(collection: &Collection, config: &Config, output_dir: &Path, progress: bool) -> Result<()> {
    // Clean and create output directory.
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;

    let template_pb = progress.then(template_spinner);
    let templates_dir = collection.root.join("templates");
    let site = Site::new(collection, config, &templates_dir)?;
    finish(template_pb, "Templates compiled");

    if !progress {
        info!("Building {} documents", site.documents.len());
    }

    let doc_pb = progress.then(|| document_progress(site.documents.len()));
    let doc_count = AtomicUsize::new(0);

    let results: Vec<Result<()>> = site.documents
        .par_iter()
        .map(|doc| {
            let result = build_document(doc, config, &site, output_dir);
            if let Some(pb) = &doc_pb {
                pb.set_position(doc_count.fetch_add(1, Ordering::Relaxed) as u64 + 1);
            }
            result
        })
        .collect();

    for result in results {
        result?;
    }
    finish(doc_pb, &format!("Built {} documents", site.documents.len()));

    site.finish(collection, config, output_dir, progress)
}

/// Build the collection with incremental support.
pub fn build_incremental(
    collection: &Collection,
    config: &Config,
    output_dir: &Path,
) -> Result<IncrementalBuildResult> {
    let result = build_incremental_inner(collection, config, output_dir, false)?;
    result.log();
    Ok(result)
}

/// Build the collection with incremental support and progress bars.
pub fn build_incremental_with_progress(
    collection: &Collection,
    config: &Config,
    output_dir: &Path,
) -> Result<IncrementalBuildResult> {
    build_incremental_inner(collection, config, output_dir, true)
}

fn build_incremental_inner(
    collection: &Collection,
    config: &Config,
    output_dir: &Path,
    progress: bool,
) -> Result<IncrementalBuildResult> {
    // Load existing cache.
    let mut cache = BuildCache::load(&collection.root);

    // Check if templates have changed.
    let template_pb = progress.then(template_spinner);
    let templates_dir = collection.root.join("templates");
    let template_hash = hash_templates(&templates_dir)?;

    if cache.templates_changed(&template_hash) {
        info!("Templates changed, forcing full rebuild");
        cache.clear();
    }

    fs::create_dir_all(output_dir)?;

    let site = Site::new(collection, config, &templates_dir)?;
    finish(template_pb, "Templates compiled");

    let result = Mutex::new(IncrementalBuildResult::new());
    result.lock().unwrap().total = site.documents.len();

    let doc_pb = progress.then(|| incremental_progress(site.documents.len()));
    let doc_count = AtomicUsize::new(0);

    // Build documents in parallel, checking cache.
    let build_results: Vec<Result<Option<(PathBuf, String, String)>>> = site.documents
        .par_iter()
        .map(|doc| {
            let output_path = output_dir.join(doc.slug()).join("index.html");
            let status = cache.check(&doc.source_path, &doc.content_hash, &output_path);

            let res = if status == CacheStatus::Fresh {
                debug!("Skipping (cached): {}", doc.source_path.display());
                result.lock().unwrap().skipped += 1;
                Ok(None)
            } else {
                build_document(doc, config, &site, output_dir).map(|()| {
                    result.lock().unwrap().rebuilt += 1;
                    Some((
                        doc.source_path.clone(),
                        doc.content_hash.clone(),
                        output_path.to_string_lossy().to_string(),
                    ))
                })
            };

            if let Some(pb) = &doc_pb {
                pb.set_position(doc_count.fetch_add(1, Ordering::Relaxed) as u64 + 1);
            }
            res
        })
        .collect();

    // Check for errors and update cache.
    for res in build_results {
        if let Some((source_path, content_hash, output_path)) = res? {
            cache.update(source_path, content_hash, output_path.into());
        }
    }

    {
        let interim = result.lock().unwrap();
        finish(
            doc_pb,
            &format!("Rebuilt {}, skipped {} documents", interim.rebuilt, interim.skipped),
        );
    }

    site.finish(collection, config, output_dir, progress)?;

    // Prune deleted documents from cache.
    let source_paths: Vec<_> = site.documents.iter().map(|d| d.source_path.clone()).collect();
    cache.prune(&source_paths);

    // Update template hash and save cache.
    cache.set_template_hash(template_hash);
    cache.save(&collection.root)?;

    Ok(result.into_inner().unwrap())
}

/// Every file a build writes other than pages and static files,
/// relative to the output directory.
pub fn generated_files(config: &Config) -> Vec<&'static str> {
    let features = config.build.features;
    let mut files: Vec<&'static str> = site_assets(config).iter().map(|asset| asset.path).collect();
    if features.contains(BuildFeatures::SEARCH) {
        files.push("search-index.json");
    }
    if features.contains(BuildFeatures::FEEDS) {
        files.extend(["rss.xml", "atom.xml", "feed.json"]);
    }
    if features.contains(BuildFeatures::SITEMAP) {
        files.push("sitemap.xml");
    }
    files
}

/// What every kind of build sets up before rendering documents.
struct Site<'a> {
    engine: TemplateEngine,
    assets: Vec<Asset>,
    documents: Vec<&'a Document>,
    index: SiteIndex,
}

impl<'a> Site<'a> {
    fn new(collection: &'a Collection, config: &Config, templates_dir: &Path) -> Result<Self> {
        // Assets are generated up front so pages can carry their hashes.
        let assets = site_assets(config);
        let engine = TemplateEngine::new(templates_dir)?
            .with_seed(config.build.seed)
            .with_integrity(integrity_map(&assets));

        let documents = if config.build.features.contains(BuildFeatures::DRAFTS) {
            collection.all_sorted()
        } else {
            collection.published()
        };

        let index = SiteIndex::new(collection, config)?;

        Ok(Self { engine, assets, documents, index })
    }

    /// Write everything that is not a document page.
    fn finish(&self, collection: &Collection, config: &Config, output_dir: &Path, progress: bool) -> Result<()> {
        let features = config.build.features;

        build_index(&self.documents, config, &self.engine, output_dir)?;
        build_tag_pages(collection, config, &self.engine, output_dir, features.contains(BuildFeatures::DRAFTS))?;

        let static_dir = collection.root.join("static");
        if static_dir.exists() {
            let pb = progress.then(asset_spinner);
            copy_static(&static_dir, output_dir)?;
            finish(pb, "Static assets copied");
        }

        for asset in &self.assets {
            fs::write(output_dir.join(asset.path), &asset.contents)?;
        }

        if features.contains(BuildFeatures::SEARCH) {
            let pb = progress.then(search_spinner);
            let search_index = crate::search::SearchIndex::build(collection);
            let search_json = rmx::serde_json::to_string(&search_index)?;
            fs::write(output_dir.join("search-index.json"), search_json)?;
            finish(pb, "Search index built");
        }

        if features.contains(BuildFeatures::FEEDS) {
            let pb = progress.then(feed_spinner);
            fs::write(output_dir.join("rss.xml"), generate_rss(collection, config)?)?;
            fs::write(output_dir.join("atom.xml"), crate::feeds::generate_atom(collection, config)?)?;
            fs::write(output_dir.join("feed.json"), crate::feeds::generate_json_feed(collection, config)?)?;
            finish(pb, "Feeds written");
        }

        if features.contains(BuildFeatures::SITEMAP) {
            fs::write(output_dir.join("sitemap.xml"), generate_sitemap(collection, config)?)?;
        }

        // Last, so it sees every other file.
        if features.contains(BuildFeatures::COMPRESS) {
            let pb = progress.then(compress_spinner);
            compress_output(output_dir)?;
            finish(pb, "Output compressed");
        }

        Ok(())
    }
}

fn finish(pb: Option<ProgressBar>, message: &str) {
    if let Some(pb) = pb {
        finish_with_check(&pb, message);
    }
}

/// Build a single document.
fn build_document(doc: &Document, config: &Config, site: &Site, output_dir: &Path) -> Result<()> {
    let engine = &site.engine;
    debug!("Building: {}", doc.source_path.display());

    let expanded = config.build.features.contains(BuildFeatures::SHORTCODES)
        .then(|| crate::shortcode::expand(&doc.content));
    let markdown = expanded.as_ref().map_or(doc.content.as_str(), |e| e.markdown.as_str());

    // Render markdown with optional syntax highlighting.
    let html_content = if config.highlight.enabled {
        render_markdown_highlighted(markdown, &config.highlight.to_options())
    } else {
        render_markdown(markdown)
    };
    let html_content = match &expanded {
        Some(expanded) => expanded.finish(&html_content),
        None => html_content,
    };
    let html_content = site.index.rewrite_source_links(doc, &html_content);
    let html_content = site.index.add_image_dimensions(doc, &html_content);

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
    let rendered = engine.render_first(&["index.html", "default.html"], &context)?;

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
        let rendered = engine.render_first(&["tag.html", "default.html"], &context)?;

        let tag_dir = tags_dir.join(&tag);
        fs::create_dir_all(&tag_dir)?;
        fs::write(tag_dir.join("index.html"), rendered)?;
    }

    Ok(())
}

/// Copy static assets to output.
fn copy_static(static_dir: &Path, output_dir: &Path) -> Result<()> {
    use rmx::walkdir::WalkDir;

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
    let now = config.build_time();

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

pub(crate) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
