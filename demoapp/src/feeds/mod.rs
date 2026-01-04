//! Feed generation for Atom and JSON Feed formats.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use rustmax::jiff::Zoned;
use crate::collection::{Collection, Config};
use crate::Result;

/// Generate an Atom 1.0 feed.
pub fn generate_atom(collection: &Collection, config: &Config) -> Result<String> {
    let documents = collection.published();
    let now = Zoned::now();
    let updated = now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();

    let mut entries = String::new();
    for doc in documents.iter().take(20) {
        let date = doc
            .frontmatter
            .date
            .map(|d| format!("{}T00:00:00Z", d))
            .unwrap_or_else(|| updated.clone());

        let excerpt = doc.excerpt(&config.content.excerpt_separator, 500);
        let url = format!("{}{}", config.collection.base_url, doc.url_path());
        let author = doc.frontmatter.author
            .as_deref()
            .unwrap_or(&config.collection.author);

        // Categories from tags.
        let categories: String = doc.frontmatter.tags
            .iter()
            .map(|t| format!("    <category term=\"{}\" />\n", xml_escape(t)))
            .collect();

        entries.push_str(&format!(
            r#"  <entry>
    <title>{}</title>
    <link href="{}" />
    <id>{}</id>
    <updated>{}</updated>
    <author>
      <name>{}</name>
    </author>
{}    <summary type="html"><![CDATA[{}]]></summary>
  </entry>
"#,
            xml_escape(&doc.frontmatter.title),
            xml_escape(&url),
            xml_escape(&url),
            date,
            xml_escape(author),
            categories,
            excerpt
        ));
    }

    let feed = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{}</title>
  <link href="{}" />
  <link href="{}/atom.xml" rel="self" />
  <id>{}/</id>
  <updated>{}</updated>
  <subtitle>{}</subtitle>
  <author>
    <name>{}</name>
  </author>
{}
</feed>
"#,
        xml_escape(&config.collection.title),
        xml_escape(&config.collection.base_url),
        xml_escape(&config.collection.base_url),
        xml_escape(&config.collection.base_url),
        updated,
        xml_escape(&config.collection.description),
        xml_escape(&config.collection.author),
        entries
    );

    Ok(feed)
}

/// JSON Feed 1.1 structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFeed {
    pub version: String,
    pub title: String,
    pub home_page_url: String,
    pub feed_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<JsonFeedAuthor>,
    pub items: Vec<JsonFeedItem>,
}

/// JSON Feed author.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFeedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// JSON Feed item.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFeedItem {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<JsonFeedAuthor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Generate a JSON Feed 1.1.
pub fn generate_json_feed(collection: &Collection, config: &Config) -> Result<String> {
    let documents = collection.published();

    let items: Vec<JsonFeedItem> = documents
        .iter()
        .take(20)
        .map(|doc| {
            let url = format!("{}{}", config.collection.base_url, doc.url_path());
            let date = doc.frontmatter.date.map(|d| format!("{}T00:00:00Z", d));
            let author = doc.frontmatter.author
                .as_deref()
                .unwrap_or(&config.collection.author);

            JsonFeedItem {
                id: url.clone(),
                url,
                title: doc.frontmatter.title.clone(),
                content_text: Some(doc.content.clone()),
                content_html: None,
                summary: Some(doc.excerpt(&config.content.excerpt_separator, 300)),
                date_published: date.clone(),
                date_modified: date,
                authors: vec![JsonFeedAuthor {
                    name: author.to_string(),
                    url: None,
                }],
                tags: doc.frontmatter.tags.clone(),
            }
        })
        .collect();

    let feed = JsonFeed {
        version: "https://jsonfeed.org/version/1.1".to_string(),
        title: config.collection.title.clone(),
        home_page_url: config.collection.base_url.clone(),
        feed_url: format!("{}/feed.json", config.collection.base_url),
        description: if config.collection.description.is_empty() {
            None
        } else {
            Some(config.collection.description.clone())
        },
        icon: None,
        favicon: None,
        authors: vec![JsonFeedAuthor {
            name: config.collection.author.clone(),
            url: None,
        }],
        items,
    };

    let json = rustmax::serde_json::to_string_pretty(&feed)?;
    Ok(json)
}

/// Escape special XML characters.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collection::Document;
    use std::path::PathBuf;

    fn make_doc(title: &str, content: &str, date: Option<&str>) -> Document {
        let date_line = date.map(|d| format!("date = \"{}\"\n", d)).unwrap_or_default();
        let raw = format!(
            "---\ntitle = \"{}\"\n{}tags = [\"test\"]\n---\n{}",
            title, date_line, content
        );
        Document::parse(PathBuf::from("test.md"), &raw).unwrap()
    }

    fn make_config() -> Config {
        Config {
            collection: crate::collection::CollectionConfig {
                title: "Test Blog".to_string(),
                base_url: "https://example.com".to_string(),
                description: "A test blog".to_string(),
                author: "Test Author".to_string(),
                language: "en".to_string(),
            },
            build: Default::default(),
            content: Default::default(),
            server: Default::default(),
            highlight: Default::default(),
        }
    }

    #[test]
    fn test_atom_generation() {
        let docs = vec![
            make_doc("First Post", "Hello world", Some("2024-01-15")),
            make_doc("Second Post", "Another post", Some("2024-01-16")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let atom = generate_atom(&collection, &config).unwrap();

        assert!(atom.contains("<?xml version"));
        assert!(atom.contains("<feed xmlns=\"http://www.w3.org/2005/Atom\">"));
        assert!(atom.contains("<title>Test Blog</title>"));
        assert!(atom.contains("<title>First Post</title>"));
        assert!(atom.contains("<title>Second Post</title>"));
        assert!(atom.contains("https://example.com"));
        assert!(atom.contains("<author>"));
        assert!(atom.contains("<name>Test Author</name>"));
    }

    #[test]
    fn test_atom_escaping() {
        // Test special character escaping in output.
        // Note: title must be valid TOML, so we escape quotes there.
        let docs = vec![
            make_doc("Post with <special> & chars", "Content", Some("2024-01-15")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let atom = generate_atom(&collection, &config).unwrap();

        assert!(atom.contains("&lt;special&gt;"));
        assert!(atom.contains("&amp;"));
    }

    #[test]
    fn test_atom_categories() {
        let docs = vec![
            make_doc("Tagged Post", "Content", Some("2024-01-15")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let atom = generate_atom(&collection, &config).unwrap();

        assert!(atom.contains("<category term=\"test\""));
    }

    #[test]
    fn test_json_feed_generation() {
        let docs = vec![
            make_doc("First Post", "Hello world", Some("2024-01-15")),
            make_doc("Second Post", "Another post", Some("2024-01-16")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let json = generate_json_feed(&collection, &config).unwrap();

        assert!(json.contains("https://jsonfeed.org/version/1.1"));
        assert!(json.contains("Test Blog"));
        assert!(json.contains("First Post"));
        assert!(json.contains("Second Post"));
        assert!(json.contains("https://example.com"));
    }

    #[test]
    fn test_json_feed_structure() {
        let docs = vec![
            make_doc("Test Post", "Content here", Some("2024-01-15")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let json_str = generate_json_feed(&collection, &config).unwrap();

        // Parse to verify structure.
        let feed: JsonFeed = rustmax::serde_json::from_str(&json_str).unwrap();

        assert_eq!(feed.version, "https://jsonfeed.org/version/1.1");
        assert_eq!(feed.title, "Test Blog");
        assert_eq!(feed.items.len(), 1);
        assert_eq!(feed.items[0].title, "Test Post");
        assert!(feed.items[0].tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_json_feed_item_fields() {
        let docs = vec![
            make_doc("My Post", "This is the content", Some("2024-06-01")),
        ];

        let collection = Collection {
            root: PathBuf::from("."),
            documents: docs,
        };

        let config = make_config();
        let json_str = generate_json_feed(&collection, &config).unwrap();
        let feed: JsonFeed = rustmax::serde_json::from_str(&json_str).unwrap();

        let item = &feed.items[0];
        assert_eq!(item.title, "My Post");
        // URL is based on slug derived from filename (test.md -> test).
        assert!(item.url.contains("/test/"));
        assert_eq!(item.date_published, Some("2024-06-01T00:00:00Z".to_string()));
        assert!(item.content_text.as_ref().unwrap().contains("This is the content"));
    }

    #[test]
    fn test_empty_collection_feeds() {
        let collection = Collection {
            root: PathBuf::from("."),
            documents: vec![],
        };

        let config = make_config();

        let atom = generate_atom(&collection, &config).unwrap();
        assert!(atom.contains("<feed"));
        assert!(!atom.contains("<entry>"));

        let json = generate_json_feed(&collection, &config).unwrap();
        let feed: JsonFeed = rustmax::serde_json::from_str(&json).unwrap();
        assert!(feed.items.is_empty());
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("hello"), "hello");
        assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
        assert_eq!(xml_escape("\"quote\""), "&quot;quote&quot;");
        assert_eq!(xml_escape("it's"), "it&apos;s");
    }
}
