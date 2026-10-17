//! The news posts, built by megaspace-pipeliner so megaspace.dev can syndicate them.

use anyhow::{Context, Result as AnyResult, anyhow as A};
use megaspace_pipeliner::{DocSet, PostsConfig, Site};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Build the posts page, a page per post, and the RSS and JSON feeds into `out_dir`.
///
/// The pages link to the rest of the site in `www_dir`, which is deployed alongside them.
pub fn build_posts(news_dir: &Path, posts_dir: &Path, www_dir: &Path, out_dir: &Path) -> AnyResult<()> {
    let doc_sets = [DocSet {
        input_dir: news_dir,
        mount: "",
        static_assets: &[],
        extra_context: &[],
        link_context: &[],
        posts: Some(PostsConfig {
            site_title: "Rustmax",
            site_description: "Updates from rustmax.net",
            author: "Brian Anderson",
            language: "en",
            base_url: "https://rustmax.net",
            categories: &["crates", "tips", "news"],
            excerpt_bytes: 600,
            posts_dir: Some(posts_dir),
            syndication: None,
        }),
    }];

    Site {
        output_dir: out_dir,
        doc_sets: &doc_sets,
        deployed_with: &[www_dir],
    }.build()
}

/// The parts of pipeliner's `posts.json` the latest post line needs.
#[derive(Deserialize)]
struct JsonFeed {
    items: Vec<JsonItem>,
}

#[derive(Deserialize)]
struct JsonItem {
    title: String,
    /// RFC 3339.
    date_published: String,
    #[serde(rename = "_megaspace")]
    megaspace: JsonItemExtension,
}

#[derive(Deserialize)]
struct JsonItemExtension {
    /// The post's file stem, which names its page.
    name: String,
}

/// Write the line naming the newest post, which the home page loads, to `out_path`.
pub fn generate_latest_post(posts_json: &Path, tera: &tera::Tera, out_path: &Path) -> AnyResult<()> {
    let json = fs::read_to_string(posts_json)
        .with_context(|| format!("failed to read {}", posts_json.display()))?;
    let feed: JsonFeed = serde_json::from_str(&json).context("parsing posts.json")?;
    let latest = feed.items.first().ok_or(A!("no posts"))?;
    let date = latest.date_published.get(..10).ok_or(A!("bad date {}", latest.date_published))?;

    let mut context = tera::Context::new();
    context.insert("title", &latest.title);
    context.insert("date", date);
    context.insert("name", &latest.megaspace.name);

    let rendered = tera.render("latest-post.template.html", &context)?;
    fs::write(out_path, rendered)?;
    eprintln!("wrote {}", out_path.display());

    Ok(())
}
