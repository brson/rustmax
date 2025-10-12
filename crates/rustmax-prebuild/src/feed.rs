use anyhow::{Context, Result as AnyResult, anyhow as A};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub date: String,
    pub category: String,
    pub title: String,
    pub summary: String,
    pub content_md: String,
    pub content_html: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Crates,
    Tips,
    News,
}

impl Category {
    fn from_str(s: &str) -> AnyResult<Self> {
        match s {
            "crates" => Ok(Category::Crates),
            "tips" => Ok(Category::Tips),
            "news" => Ok(Category::News),
            _ => Err(A!("invalid category: {}", s)),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Category::Crates => "crates",
            Category::Tips => "tips",
            Category::News => "news",
        }
    }
}

pub fn parse_posts(posts_dir: &Path) -> AnyResult<Vec<Post>> {
    let filename_regex = Regex::new(r"^(\d{4}-\d{2}-\d{2})-(.+)\.md$")?;

    let mut posts = Vec::new();

    if !posts_dir.exists() {
        return Ok(posts);
    }

    for entry in fs::read_dir(posts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or(A!("invalid filename"))?;

        let Some(captures) = filename_regex.captures(filename) else {
            continue;
        };

        let date_str = captures.get(1).unwrap().as_str();
        let slug = captures.get(2).unwrap().as_str().to_string();

        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .context("parsing date")?;

        let content = fs::read_to_string(&path)?;
        let (frontmatter, content_md) = parse_frontmatter(&content)?;

        let title = frontmatter
            .get("title")
            .ok_or(A!("missing title in frontmatter"))?
            .clone();

        let summary = frontmatter
            .get("summary")
            .ok_or(A!("missing summary in frontmatter"))?
            .clone();

        let category_str = frontmatter
            .get("category")
            .ok_or(A!("missing category in frontmatter"))?;

        let category = Category::from_str(category_str)?;

        let content_html = comrak::markdown_to_html(&content_md, &Default::default());

        posts.push(Post {
            slug,
            date: date.format("%Y-%m-%d").to_string(),
            category: category.as_str().to_string(),
            title,
            summary,
            content_md,
            content_html,
        });
    }

    // Sort by date descending (newest first).
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

fn parse_frontmatter(content: &str) -> AnyResult<(BTreeMap<String, String>, String)> {
    let mut lines = content.lines();

    // First line should be "---".
    let first = lines.next().ok_or(A!("empty file"))?;
    if first.trim() != "---" {
        return Err(A!("missing frontmatter start"));
    }

    let mut frontmatter = BTreeMap::new();
    let mut body_lines = Vec::new();
    let mut in_frontmatter = true;

    for line in lines {
        if in_frontmatter {
            if line.trim() == "---" {
                in_frontmatter = false;
                continue;
            }

            // Parse key: value.
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').to_string();
                frontmatter.insert(key, value);
            }
        } else {
            body_lines.push(line);
        }
    }

    let body = body_lines.join("\n");

    Ok((frontmatter, body))
}

pub fn generate_feed_page(
    posts: &[Post],
    tera: &tera::Tera,
    out_dir: &Path,
) -> AnyResult<()> {
    let mut context = tera::Context::new();
    context.insert("posts", posts);

    let rendered = tera.render("feed.template.html", &context)?;

    let feed_path = out_dir.join("feed.html");
    fs::write(&feed_path, rendered)?;
    eprintln!("wrote {}", feed_path.display());

    Ok(())
}

pub fn generate_latest_post(
    posts: &[Post],
    tera: &tera::Tera,
    out_dir: &Path,
) -> AnyResult<()> {
    if posts.is_empty() {
        return Ok(());
    }

    let latest = &posts[0];

    let mut context = tera::Context::new();
    context.insert("title", &latest.title);
    context.insert("date", &latest.date);
    context.insert("category", &latest.category);
    context.insert("summary", &latest.summary);
    context.insert("content", &latest.content_html);
    context.insert("slug", &latest.slug);

    let rendered = tera.render("latest-post.template.html", &context)?;

    let latest_path = out_dir.join("latest-post.html");
    fs::write(&latest_path, rendered)?;
    eprintln!("wrote {}", latest_path.display());

    Ok(())
}

pub fn generate_rss(posts: &[Post], out_dir: &Path, base_url: &str) -> AnyResult<()> {
    let mut rss = String::new();
    rss.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    rss.push('\n');
    rss.push_str(r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">"#);
    rss.push('\n');
    rss.push_str("  <channel>\n");
    rss.push_str("    <title>Rustmax</title>\n");
    rss.push_str(&format!("    <link>{}/feed.html</link>\n", base_url));
    rss.push_str("    <description>Updates from Rustmax - curated Rust crates, tips, and news</description>\n");
    rss.push_str(&format!("    <atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\" />\n", base_url));

    for post in posts {
        rss.push_str("    <item>\n");
        rss.push_str(&format!("      <title>{}</title>\n", escape_xml(&post.title)));
        rss.push_str(&format!("      <link>{}/feed.html#{}</link>\n", base_url, post.slug));
        rss.push_str(&format!("      <guid>{}/feed.html#{}</guid>\n", base_url, post.slug));
        let date = chrono::NaiveDate::parse_from_str(&post.date, "%Y-%m-%d").unwrap();
        rss.push_str(&format!("      <pubDate>{}</pubDate>\n", format_rfc822_date(date)));
        rss.push_str(&format!("      <category>{}</category>\n", &post.category));
        rss.push_str(&format!("      <description><![CDATA[{}]]></description>\n", post.content_html));
        rss.push_str("    </item>\n");
    }

    rss.push_str("  </channel>\n");
    rss.push_str("</rss>\n");

    let rss_path = out_dir.join("feed.xml");
    fs::write(&rss_path, rss)?;
    eprintln!("wrote {}", rss_path.display());

    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn format_rfc822_date(date: chrono::NaiveDate) -> String {
    // Convert to DateTime at midnight UTC.
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    let datetime = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(datetime, chrono::Utc);
    datetime.format("%a, %d %b %Y %H:%M:%S %z").to_string()
}
