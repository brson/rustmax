//! Integration tests for anthology using tempfile.

use rmx::tempfile::tempdir;
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

    // Write default template. The variables here have to match what
    // TemplateEngine puts in the context, or the pages render empty.
    let template = r#"<!DOCTYPE html>
<html>
<head>
<title>{{ title }}</title>
<meta name="generator" content="{{ generator }}" data-build="{{ build_id }}">
</head>
<body>
{% if is_index or is_tag_page %}
<h1>{{ site_title }}</h1>
<ul>
{% for doc in documents %}
<li><a href="{{ doc.url }}">{{ doc.title }}</a></li>
{% endfor %}
</ul>
{% else %}
<h1>{{ title }}</h1>
{{ content | safe }}
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
    let json = rmx::serde_json::to_string_pretty(&export).unwrap();

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

/// Run `anthology init` then `anthology build` the way a user would,
/// against the templates that `init` writes.
///
/// The other tests here use a stub template, so they miss anything the real
/// templates rely on, like the custom `date_format` filter.
#[test]
fn init_then_build_renders_with_the_generated_templates() {
    use std::process::Command;

    let dir = tempdir().unwrap();
    let root = dir.path();
    let anthology = env!("CARGO_BIN_EXE_anthology");

    let init = Command::new(anthology)
        .args(["init", "."])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let build = Command::new(anthology)
        .arg("build")
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build.stderr)
    );

    let document = fs::read_to_string(root.join("output/welcome/index.html")).unwrap();
    let index = fs::read_to_string(root.join("output/index.html")).unwrap();
    let tag = fs::read_to_string(root.join("output/tags/welcome/index.html")).unwrap();

    // The generated template links these stylesheets; the built-in one does not,
    // so this catches pages that silently fall back to the built-in template.
    for (name, page) in [("document", &document), ("index", &index), ("tag", &tag)] {
        assert!(
            page.contains("/highlight.css"),
            "{name} page did not use the collection's template"
        );
    }

    // The document date went through the custom `date_format` filter.
    assert!(document.contains("January 01, 2024"), "date filter did not run");
}

/// Every file of a built site, keyed by its path relative to the output
/// directory, so that two builds can be compared byte for byte.
fn output_snapshot(dir: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    rmx::walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let rel = entry.path().strip_prefix(dir).unwrap().display().to_string();
            (rel, fs::read(entry.path()).unwrap())
        })
        .collect()
}

fn seeded_collection(root: &Path, seed: Option<u64>) {
    create_test_collection(root);
    if let Some(seed) = seed {
        let config = fs::read_to_string(root.join("anthology.toml")).unwrap();
        fs::write(
            root.join("anthology.toml"),
            config.replace("output_dir = \"output\"", &format!("output_dir = \"output\"\nseed = {seed}")),
        )
        .unwrap();
    }
    create_document(root, "alpha", "Alpha", "The first document, about rust.", false);
    create_document(root, "beta", "Beta", "The second document, about search.", false);
}

fn build_twice(seed: Option<u64>) -> (
    std::collections::BTreeMap<String, Vec<u8>>,
    std::collections::BTreeMap<String, Vec<u8>>,
) {
    let mut snapshots = Vec::new();
    for _ in 0..2 {
        let dir = tempdir().unwrap();
        seeded_collection(dir.path(), seed);

        let config = anthology::collection::Config::load(dir.path()).unwrap();
        let collection = anthology::collection::Collection::load(dir.path(), &config).unwrap();
        let out = dir.path().join("output");
        anthology::build::build(&collection, &config, &out, false).unwrap();

        snapshots.push(output_snapshot(&out));
    }
    (snapshots.remove(0), snapshots.remove(0))
}

#[test]
fn a_seeded_build_is_reproducible() {
    let (first, second) = build_twice(Some(1234));

    assert!(!first.is_empty(), "the build produced no files");
    assert_eq!(
        first.keys().collect::<Vec<_>>(),
        second.keys().collect::<Vec<_>>(),
    );

    for (path, bytes) in &first {
        // `build_time` is a wall-clock timestamp and is not seeded, so a page
        // carrying it can still differ. Everything else must match.
        // See ROADMAP.md: pinning it is the remaining piece.
        if path.ends_with(".json") || !path.ends_with(".html") {
            assert_eq!(bytes, &second[path], "{path} differs between builds");
        }
    }
}

#[test]
fn the_build_id_is_fixed_by_the_seed_and_random_without_one() {
    fn build_ids(seed: Option<u64>) -> (String, String) {
        let (first, second) = build_twice(seed);
        let read = |files: &std::collections::BTreeMap<String, Vec<u8>>| {
            let html = String::from_utf8(files["index.html"].clone()).unwrap();
            let start = html.find("data-build=\"").expect("no build id in {html}") + 12;
            html[start..].split('"').next().unwrap().to_owned()
        };
        (read(&first), read(&second))
    }

    let (a, b) = build_ids(Some(99));
    assert_eq!(a, b, "a seeded build changed its build id");
    assert_eq!(a.len(), 12);

    let (a, b) = build_ids(None);
    assert_ne!(a, b, "an unseeded build repeated its build id");
}

#[test]
fn the_search_index_does_not_depend_on_hash_order() {
    let (first, second) = build_twice(None);

    let index = "search-index.json";
    assert!(first.contains_key(index), "no search index in {:?}", first.keys());
    assert_eq!(
        first[index], second[index],
        "the search index differs between two builds of identical content",
    );

    // Equal bytes could still be luck, since both builds ran in one process.
    // The guarantee is that the terms are written in sorted order.
    let parsed: rmx::serde_json::Value = rmx::serde_json::from_slice(&first[index]).unwrap();
    let terms: Vec<&String> = parsed["word_index"].as_object().unwrap().keys().collect();

    assert!(terms.len() > 1, "expected an indexed vocabulary, got {terms:?}");
    assert!(terms.is_sorted(), "search index terms are not sorted: {terms:?}");
}

#[test]
fn a_collection_can_be_configured_in_json5() {
    let dir = tempdir().unwrap();
    create_test_collection(dir.path());
    fs::remove_file(dir.path().join("anthology.toml")).unwrap();
    fs::write(
        dir.path().join("anthology.json5"),
        r#"{
            // A collection written by another tool.
            collection: { title: "From JSON5", base_url: "https://json5.example.com" },
            build: { output_dir: "output" },
        }"#,
    )
    .unwrap();
    create_document(dir.path(), "one", "One", "Body text.", false);

    let config = anthology::collection::Config::load(dir.path()).unwrap();
    assert_eq!(config.collection.title, "From JSON5");

    let collection = anthology::collection::Collection::load(dir.path(), &config).unwrap();
    let out = dir.path().join("output");
    anthology::build::build(&collection, &config, &out, false).unwrap();

    let index = fs::read_to_string(out.join("index.html")).unwrap();
    assert!(index.contains("From JSON5"), "{index}");
}

#[test]
fn a_built_site_can_be_archived_and_unpacked() {
    let dir = tempdir().unwrap();
    seeded_collection(dir.path(), None);

    let config = anthology::collection::Config::load(dir.path()).unwrap();
    let collection = anthology::collection::Collection::load(dir.path(), &config).unwrap();
    let out = dir.path().join("output");
    anthology::build::build(&collection, &config, &out, false).unwrap();

    let archive = dir.path().join("site.tar.gz");
    let options = anthology::export::ArchiveOptions::default();
    let stats = anthology::export::archive_directory(&out, &archive, &options).unwrap();

    assert_eq!(stats.files, output_snapshot(&out).len());

    let entries = anthology::export::list_archive(&archive).unwrap();
    assert!(
        entries.iter().any(|e| e.path.ends_with("index.html")),
        "{entries:?}",
    );
}

#[test]
fn check_reports_a_rust_code_block_that_does_not_parse() {
    let dir = tempdir().unwrap();
    create_test_collection(dir.path());
    create_document(
        dir.path(),
        "broken",
        "Broken",
        "Here is some code:\n\n```rust\nfn main( {\n```\n",
        false,
    );
    create_document(
        dir.path(),
        "fine",
        "Fine",
        "And some that is fine:\n\n```rust\nlet x = 1;\n```\n",
        false,
    );

    let config = anthology::collection::Config::load(dir.path()).unwrap();
    let collection = anthology::collection::Collection::load(dir.path(), &config).unwrap();

    let lints: Vec<_> = collection
        .documents
        .iter()
        .flat_map(anthology::lint::check_document)
        .collect();

    assert_eq!(lints.len(), 1, "{lints:?}");
    assert!(lints[0].path.ends_with("broken.md"), "{:?}", lints[0]);
    assert_eq!(lints[0].level, anthology::lint::Level::Error);
}
