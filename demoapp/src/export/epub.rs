//! EPUB generation for collections.
//!
//! Creates EPUB3-compatible ebooks from document collections.

use rustmax::zip::{ZipWriter, write::SimpleFileOptions, CompressionMethod};
use rustmax::jiff::Zoned;
use std::io::{Write, Seek};
use std::path::Path;
use std::fs::File;

use crate::collection::{Collection, Config, Document};
use crate::build::render_markdown;
use crate::{Error, Result};

/// Configuration for EPUB generation.
#[derive(Debug, Clone)]
pub struct EpubConfig {
    /// Book title (defaults to collection title).
    pub title: Option<String>,
    /// Book author.
    pub author: Option<String>,
    /// Book language code (e.g., "en").
    pub language: String,
    /// Book description.
    pub description: Option<String>,
    /// Publisher name.
    pub publisher: Option<String>,
    /// Cover image path (optional).
    pub cover_image: Option<String>,
    /// Include draft documents.
    pub include_drafts: bool,
}

impl Default for EpubConfig {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            language: "en".to_string(),
            description: None,
            publisher: None,
            cover_image: None,
            include_drafts: false,
        }
    }
}

/// EPUB builder for constructing ebooks.
pub struct EpubBuilder<W: Write + Seek> {
    zip: ZipWriter<W>,
    config: EpubConfig,
    spine_items: Vec<SpineItem>,
    manifest_items: Vec<ManifestItem>,
}

#[derive(Debug)]
struct SpineItem {
    id: String,
}

#[derive(Debug)]
struct ManifestItem {
    id: String,
    href: String,
    media_type: String,
}

impl<W: Write + Seek> EpubBuilder<W> {
    /// Create a new EPUB builder.
    pub fn new(writer: W, config: EpubConfig) -> Result<Self> {
        let zip = ZipWriter::new(writer);
        Ok(Self {
            zip,
            config,
            spine_items: Vec::new(),
            manifest_items: Vec::new(),
        })
    }

    /// Write the mimetype file (must be first and uncompressed).
    fn write_mimetype(&mut self) -> Result<()> {
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);
        self.zip.start_file("mimetype", options)
            .map_err(|e| Error::build(format!("Failed to create mimetype: {}", e)))?;
        self.zip.write_all(b"application/epub+zip")
            .map_err(|e| Error::build(format!("Failed to write mimetype: {}", e)))?;
        Ok(())
    }

    /// Write the META-INF/container.xml file.
    fn write_container(&mut self) -> Result<()> {
        let options = SimpleFileOptions::default();
        self.zip.start_file("META-INF/container.xml", options)
            .map_err(|e| Error::build(format!("Failed to create container.xml: {}", e)))?;

        let container = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#;

        self.zip.write_all(container.as_bytes())
            .map_err(|e| Error::build(format!("Failed to write container.xml: {}", e)))?;
        Ok(())
    }

    /// Write the stylesheet.
    fn write_stylesheet(&mut self) -> Result<()> {
        let options = SimpleFileOptions::default();
        self.zip.start_file("OEBPS/style.css", options)
            .map_err(|e| Error::build(format!("Failed to create style.css: {}", e)))?;

        let css = r#"body {
    font-family: Georgia, serif;
    line-height: 1.6;
    margin: 2em;
}
h1, h2, h3, h4, h5, h6 {
    font-family: Helvetica, Arial, sans-serif;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
}
h1 { font-size: 2em; }
h2 { font-size: 1.5em; }
h3 { font-size: 1.25em; }
p { margin: 1em 0; }
code {
    font-family: monospace;
    background: #f4f4f4;
    padding: 0.1em 0.3em;
}
pre {
    background: #f4f4f4;
    padding: 1em;
    overflow-x: auto;
}
blockquote {
    border-left: 3px solid #ccc;
    margin-left: 0;
    padding-left: 1em;
    font-style: italic;
}
a { color: #0066cc; }
"#;

        self.zip.write_all(css.as_bytes())
            .map_err(|e| Error::build(format!("Failed to write style.css: {}", e)))?;

        self.manifest_items.push(ManifestItem {
            id: "style".to_string(),
            href: "style.css".to_string(),
            media_type: "text/css".to_string(),
        });

        Ok(())
    }

    /// Add a document to the EPUB.
    pub fn add_document(&mut self, doc: &Document, index: usize) -> Result<()> {
        let id = format!("chapter{}", index);
        let filename = format!("{}.xhtml", id);
        let options = SimpleFileOptions::default();

        self.zip.start_file(format!("OEBPS/{}", filename), options)
            .map_err(|e| Error::build(format!("Failed to create {}: {}", filename, e)))?;

        let html_content = render_markdown(&doc.content);
        let xhtml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="{}">
<head>
  <meta charset="utf-8"/>
  <title>{}</title>
  <link rel="stylesheet" type="text/css" href="style.css"/>
</head>
<body>
<h1>{}</h1>
{}
</body>
</html>"#,
            self.config.language,
            xml_escape(&doc.frontmatter.title),
            xml_escape(&doc.frontmatter.title),
            html_content
        );

        self.zip.write_all(xhtml.as_bytes())
            .map_err(|e| Error::build(format!("Failed to write {}: {}", filename, e)))?;

        self.manifest_items.push(ManifestItem {
            id: id.clone(),
            href: filename,
            media_type: "application/xhtml+xml".to_string(),
        });

        self.spine_items.push(SpineItem { id });

        Ok(())
    }

    /// Write the navigation document (EPUB3 toc).
    fn write_nav(&mut self, documents: &[&Document]) -> Result<()> {
        let options = SimpleFileOptions::default();
        self.zip.start_file("OEBPS/nav.xhtml", options)
            .map_err(|e| Error::build(format!("Failed to create nav.xhtml: {}", e)))?;

        let mut toc_items = String::new();
        for (i, doc) in documents.iter().enumerate() {
            toc_items.push_str(&format!(
                "      <li><a href=\"chapter{}.xhtml\">{}</a></li>\n",
                i,
                xml_escape(&doc.frontmatter.title)
            ));
        }

        let title = self.config.title.as_deref().unwrap_or("Table of Contents");

        let nav = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="{}">
<head>
  <meta charset="utf-8"/>
  <title>Table of Contents</title>
  <link rel="stylesheet" type="text/css" href="style.css"/>
</head>
<body>
  <nav epub:type="toc" id="toc">
    <h1>{}</h1>
    <ol>
{}    </ol>
  </nav>
</body>
</html>"#,
            self.config.language,
            xml_escape(title),
            toc_items
        );

        self.zip.write_all(nav.as_bytes())
            .map_err(|e| Error::build(format!("Failed to write nav.xhtml: {}", e)))?;

        self.manifest_items.push(ManifestItem {
            id: "nav".to_string(),
            href: "nav.xhtml".to_string(),
            media_type: "application/xhtml+xml".to_string(),
        });

        Ok(())
    }

    /// Write the content.opf package file.
    fn write_opf(&mut self, config: &Config) -> Result<()> {
        let options = SimpleFileOptions::default();
        self.zip.start_file("OEBPS/content.opf", options)
            .map_err(|e| Error::build(format!("Failed to create content.opf: {}", e)))?;

        let title = self.config.title.as_ref()
            .unwrap_or(&config.collection.title);
        let author = self.config.author.as_ref()
            .unwrap_or(&config.collection.author);
        let description = self.config.description.as_ref()
            .unwrap_or(&config.collection.description);

        let uuid = generate_uuid();
        let now = Zoned::now();
        let modified = now.strftime("%Y-%m-%dT%H:%M:%SZ").to_string();

        let mut manifest = String::new();
        for item in &self.manifest_items {
            let properties = if item.id == "nav" {
                " properties=\"nav\""
            } else {
                ""
            };
            manifest.push_str(&format!(
                "    <item id=\"{}\" href=\"{}\" media-type=\"{}\"{}/>\n",
                item.id, item.href, item.media_type, properties
            ));
        }

        let mut spine = String::new();
        for item in &self.spine_items {
            spine.push_str(&format!("    <itemref idref=\"{}\"/>\n", item.id));
        }

        let opf = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="BookId">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:identifier id="BookId">urn:uuid:{}</dc:identifier>
    <dc:title>{}</dc:title>
    <dc:creator>{}</dc:creator>
    <dc:language>{}</dc:language>
    <dc:description>{}</dc:description>
    <meta property="dcterms:modified">{}</meta>
  </metadata>
  <manifest>
{}  </manifest>
  <spine>
{}  </spine>
</package>"#,
            uuid,
            xml_escape(title),
            xml_escape(author),
            self.config.language,
            xml_escape(description),
            modified,
            manifest,
            spine
        );

        self.zip.write_all(opf.as_bytes())
            .map_err(|e| Error::build(format!("Failed to write content.opf: {}", e)))?;

        Ok(())
    }

    /// Finalize the EPUB file.
    pub fn finish(mut self, documents: &[&Document], config: &Config) -> Result<W> {
        self.write_mimetype()?;
        self.write_container()?;
        self.write_stylesheet()?;

        for (i, doc) in documents.iter().enumerate() {
            self.add_document(doc, i)?;
        }

        self.write_nav(documents)?;
        self.write_opf(config)?;

        self.zip.finish()
            .map_err(|e| Error::build(format!("Failed to finalize EPUB: {}", e)))
    }
}

/// Generate an EPUB file from a collection.
pub fn generate_epub(
    collection: &Collection,
    config: &Config,
    output_path: &Path,
    epub_config: &EpubConfig,
) -> Result<()> {
    let file = File::create(output_path)
        .map_err(|e| Error::build(format!("Failed to create EPUB file: {}", e)))?;

    let builder = EpubBuilder::new(file, epub_config.clone())?;

    let documents: Vec<&Document> = if epub_config.include_drafts {
        collection.all_sorted()
    } else {
        collection.published()
    };

    builder.finish(&documents, config)?;

    Ok(())
}

/// Escape XML special characters.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Generate a simple UUID v4.
fn generate_uuid() -> String {
    use rustmax::rand::Rng;
    let mut rng = rustmax::rand::rng();

    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);

    // Set version (4) and variant (RFC 4122).
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax::tempfile::tempdir;
    use std::io::Cursor;

    #[test]
    fn test_epub_config_default() {
        let config = EpubConfig::default();
        assert_eq!(config.language, "en");
        assert!(!config.include_drafts);
        assert!(config.title.is_none());
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("Hello & World"), "Hello &amp; World");
        assert_eq!(xml_escape("<script>"), "&lt;script&gt;");
        assert_eq!(xml_escape("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));

        // Verify format: xxxxxxxx-xxxx-4xxx-[89ab]xxx-xxxxxxxxxxxx
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);

        // Check version 4 marker.
        assert!(parts[2].starts_with('4'));
    }

    #[test]
    fn test_epub_builder_creates_valid_structure() {
        let buffer = Cursor::new(Vec::new());
        let config = EpubConfig::default();

        let mut builder = EpubBuilder::new(buffer, config).unwrap();
        builder.write_mimetype().unwrap();
        builder.write_container().unwrap();
        builder.write_stylesheet().unwrap();

        let result = builder.zip.finish().unwrap();
        let data = result.into_inner();

        // Verify ZIP structure.
        let reader = Cursor::new(data);
        let mut archive = rustmax::zip::ZipArchive::new(reader).unwrap();

        assert!(archive.by_name("mimetype").is_ok());
        assert!(archive.by_name("META-INF/container.xml").is_ok());
        assert!(archive.by_name("OEBPS/style.css").is_ok());
    }

    #[test]
    fn test_generate_epub_creates_file() {
        use crate::collection::{Collection, Config};

        let dir = tempdir().unwrap();
        let content_dir = dir.path().join("content");
        std::fs::create_dir_all(&content_dir).unwrap();

        // Create a test document.
        let doc_content = r#"---
title = "Test Document"
date = "2024-01-01"
---

# Hello

This is a test document.
"#;
        std::fs::write(content_dir.join("test.md"), doc_content).unwrap();

        // Create config.
        let config_content = r#"
[collection]
title = "Test Collection"
base_url = "http://example.com"
"#;
        std::fs::write(dir.path().join("anthology.toml"), config_content).unwrap();

        let config = Config::load(dir.path()).unwrap();
        let collection = Collection::load(dir.path(), &config).unwrap();

        let epub_path = dir.path().join("output.epub");
        let epub_config = EpubConfig::default();

        generate_epub(&collection, &config, &epub_path, &epub_config).unwrap();

        assert!(epub_path.exists());

        // Verify it's a valid ZIP.
        let file = File::open(&epub_path).unwrap();
        let mut archive = rustmax::zip::ZipArchive::new(file).unwrap();

        assert!(archive.by_name("mimetype").is_ok());
        assert!(archive.by_name("OEBPS/content.opf").is_ok());
        assert!(archive.by_name("OEBPS/nav.xhtml").is_ok());
        assert!(archive.by_name("OEBPS/chapter0.xhtml").is_ok());
    }
}
