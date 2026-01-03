//! Integration tests for anthology using tempfile.

use rustmax::tempfile::tempdir;
use std::fs;
use std::path::Path;

/// Helper to create a minimal collection structure.
fn create_test_collection(root: &Path) {
    fs::create_dir_all(root.join("content")).unwrap();
    fs::create_dir_all(root.join("templates")).unwrap();
    fs::create_dir_all(root.join("static")).unwrap();

    // Write config.
    let config = r#"
[collection]
title = "Test Collection"
base_url = "https://test.example.com"

[build]
output_dir = "output"
"#;
    fs::write(root.join("anthology.toml"), config).unwrap();

    // Write default template.
    let template = r#"<!DOCTYPE html>
<html>
<head><title>{{ title }}</title></head>
<body>
{% if document %}
<h1>{{ document.title }}</h1>
{{ content | safe }}
{% else %}
<h1>{{ collection.title }}</h1>
<ul>
{% for doc in documents %}
<li><a href="{{ doc.url }}">{{ doc.title }}</a></li>
{% endfor %}
</ul>
{% endif %}
</body>
</html>"#;
    fs::write(root.join("templates/default.html"), template).unwrap();
}

/// Helper to create a document.
fn create_document(root: &Path, name: &str, title: &str, content: &str, draft: bool) {
    let doc = format!(
        r#"---
title = "{}"
date = "2024-01-15"
draft = {}
tags = ["test"]
---

{}"#,
        title, draft, content
    );
    fs::write(root.join("content").join(format!("{}.md", name)), doc).unwrap();
}

#[test]
fn test_collection_load() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "hello", "Hello World", "This is a test.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    assert_eq!(config.collection.title, "Test Collection");

    let collection = anthology::collection::Collection::load(root, &config).unwrap();
    assert_eq!(collection.documents.len(), 1);
    assert_eq!(collection.documents[0].frontmatter.title, "Hello World");
}

#[test]
fn test_collection_multiple_documents() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "first", "First Post", "Content one.", false);
    create_document(root, "second", "Second Post", "Content two.", false);
    create_document(root, "third", "Third Post", "Content three.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    assert_eq!(collection.documents.len(), 3);

    let published = collection.published();
    assert_eq!(published.len(), 3);
}

#[test]
fn test_collection_drafts_filtered() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "published", "Published Post", "Public content.", false);
    create_document(root, "draft", "Draft Post", "Secret content.", true);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    assert_eq!(collection.documents.len(), 2);

    let published = collection.published();
    assert_eq!(published.len(), 1);
    assert_eq!(published[0].frontmatter.title, "Published Post");

    let all = collection.all_sorted();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_collection_tags() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);

    // Create documents with different tags.
    let doc1 = r#"---
title = "Rust Post"
date = "2024-01-15"
tags = ["rust", "programming"]
---
Content"#;
    fs::write(root.join("content/rust.md"), doc1).unwrap();

    let doc2 = r#"---
title = "Python Post"
date = "2024-01-16"
tags = ["python", "programming"]
---
Content"#;
    fs::write(root.join("content/python.md"), doc2).unwrap();

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let tags = collection.tags();
    assert!(tags.contains(&"rust".to_string()));
    assert!(tags.contains(&"python".to_string()));
    assert!(tags.contains(&"programming".to_string()));

    let rust_docs = collection.by_tag("rust");
    assert_eq!(rust_docs.len(), 1);

    let programming_docs = collection.by_tag("programming");
    assert_eq!(programming_docs.len(), 2);
}

#[test]
fn test_build_output() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "hello", "Hello World", "This is **bold** text.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let output_dir = root.join("output");
    anthology::build::build(&collection, &config, &output_dir, false).unwrap();

    // Check output structure.
    assert!(output_dir.exists());
    assert!(output_dir.join("index.html").exists());
    assert!(output_dir.join("hello/index.html").exists());

    // Check content.
    let index_html = fs::read_to_string(output_dir.join("index.html")).unwrap();
    assert!(index_html.contains("Test Collection"));
    assert!(index_html.contains("Hello World"));

    let doc_html = fs::read_to_string(output_dir.join("hello/index.html")).unwrap();
    assert!(doc_html.contains("Hello World"));
    assert!(doc_html.contains("<strong>bold</strong>"));
}

#[test]
fn test_build_with_static_assets() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "test", "Test", "Content", false);

    // Create static assets.
    fs::write(root.join("static/style.css"), "body { color: red; }").unwrap();
    fs::create_dir_all(root.join("static/js")).unwrap();
    fs::write(root.join("static/js/app.js"), "console.log('hello');").unwrap();

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let output_dir = root.join("output");
    anthology::build::build(&collection, &config, &output_dir, false).unwrap();

    // Check static assets copied.
    assert!(output_dir.join("style.css").exists());
    assert!(output_dir.join("js/app.js").exists());

    let css = fs::read_to_string(output_dir.join("style.css")).unwrap();
    assert!(css.contains("color: red"));
}

#[test]
fn test_build_excludes_drafts() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "published", "Published", "Public", false);
    create_document(root, "draft", "Draft", "Private", true);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let output_dir = root.join("output");
    anthology::build::build(&collection, &config, &output_dir, false).unwrap();

    // Published should exist.
    assert!(output_dir.join("published/index.html").exists());

    // Draft should not exist.
    assert!(!output_dir.join("draft/index.html").exists());
}

#[test]
fn test_build_includes_drafts_when_requested() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "published", "Published", "Public", false);
    create_document(root, "draft", "Draft", "Private", true);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let output_dir = root.join("output");
    anthology::build::build(&collection, &config, &output_dir, true).unwrap();

    // Both should exist.
    assert!(output_dir.join("published/index.html").exists());
    assert!(output_dir.join("draft/index.html").exists());
}

#[test]
fn test_search_index_build() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "rust", "Rust Programming", "Learn about memory safety.", false);
    create_document(root, "python", "Python Basics", "Learn about scripting.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    anthology::search::build_index(&collection, root).unwrap();

    // Check index file created.
    let index_path = root.join("search-index.json");
    assert!(index_path.exists());

    let index_content = fs::read_to_string(&index_path).unwrap();
    assert!(index_content.contains("Rust Programming"));
    assert!(index_content.contains("Python Basics"));
}

#[test]
fn test_rss_generation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "post", "My Post", "Some content here.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let rss = anthology::build::generate_rss(&collection, &config).unwrap();

    assert!(rss.contains("<?xml version="));
    assert!(rss.contains("<rss version=\"2.0\">"));
    assert!(rss.contains("Test Collection"));
    assert!(rss.contains("My Post"));
    assert!(rss.contains("test.example.com"));
}

#[test]
fn test_sitemap_generation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "page1", "Page One", "Content", false);
    create_document(root, "page2", "Page Two", "Content", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let sitemap = anthology::build::generate_sitemap(&collection, &config).unwrap();

    assert!(sitemap.contains("<?xml version="));
    assert!(sitemap.contains("<urlset"));
    assert!(sitemap.contains("test.example.com/page1/"));
    assert!(sitemap.contains("test.example.com/page2/"));
}

#[test]
fn test_json_export() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "test", "Test Doc", "Content here.", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let export = collection.to_export();
    let json = rustmax::serde_json::to_string_pretty(&export).unwrap();

    assert!(json.contains("Test Doc"));
    assert!(json.contains("\"slug\": \"test\""));
}

#[test]
fn test_config_defaults() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("content")).unwrap();

    // Minimal config.
    fs::write(root.join("anthology.toml"), "[collection]\n").unwrap();

    let config = anthology::collection::Config::load(root).unwrap();

    assert_eq!(config.collection.title, "My Collection");
    assert_eq!(config.build.output_dir, "output");
    assert_eq!(config.content.default_template, "default.html");
    assert_eq!(config.server.port, 3000);
}

#[test]
fn test_document_content_hash() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "doc", "Doc", "Content", false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let doc = &collection.documents[0];

    // Hash should be a valid blake3 hex string (64 chars).
    assert_eq!(doc.content_hash.len(), 64);
    assert!(doc.content_hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_document_reading_time() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);

    // Create document with ~400 words (should be 2 minutes).
    let words: String = (0..400).map(|i| format!("word{} ", i)).collect();
    create_document(root, "long", "Long Post", &words, false);

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    let doc = &collection.documents[0];
    assert_eq!(doc.reading_time(), 2);
}

#[test]
fn test_empty_collection() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    // No documents created.

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    assert!(collection.documents.is_empty());
    assert!(collection.published().is_empty());
    assert!(collection.tags().is_empty());
}

#[test]
fn test_nested_documents() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    create_test_collection(root);
    create_document(root, "top-level", "Top Level", "Content", false);

    // Create nested document in subdirectory.
    fs::create_dir_all(root.join("content/nested")).unwrap();
    fs::write(
        root.join("content/nested/deep.md"),
        "---\ntitle = \"Deep Post\"\ndate = \"2024-01-01\"\n---\nNested content",
    )
    .unwrap();

    let config = anthology::collection::Config::load(root).unwrap();
    let collection = anthology::collection::Collection::load(root, &config).unwrap();

    // Both documents should be loaded.
    assert_eq!(collection.documents.len(), 2);

    let titles: Vec<_> = collection.documents.iter().map(|d| &d.frontmatter.title).collect();
    assert!(titles.contains(&&"Top Level".to_string()));
    assert!(titles.contains(&&"Deep Post".to_string()));
}
