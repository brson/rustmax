//! Table of contents generation from markdown headings.
//!
//! Extracts headings from markdown content and generates hierarchical
//! table of contents with anchor links.

use rustmax::prelude::*;
use rustmax::regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Regex to match markdown headings.
static HEADING_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(#{1,6})\s+(.+?)(?:\s*\{#([a-zA-Z0-9_-]+)\})?\s*$").expect("invalid regex")
});

/// Regex to match HTML heading tags (matches each level separately since backrefs not supported).
static HTML_H1_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h1[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h1>"#).expect("invalid regex"));
static HTML_H2_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h2[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h2>"#).expect("invalid regex"));
static HTML_H3_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h3[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h3>"#).expect("invalid regex"));
static HTML_H4_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h4[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h4>"#).expect("invalid regex"));
static HTML_H5_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h5[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h5>"#).expect("invalid regex"));
static HTML_H6_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h6[^>]*(?:id="([^"]*)")?[^>]*>(.*?)</h6>"#).expect("invalid regex"));

/// A heading extracted from content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Heading level (1-6).
    pub level: u8,
    /// Heading text (plain text, no markdown).
    pub text: String,
    /// Anchor ID for linking.
    pub id: String,
}

impl Heading {
    /// Create a new heading.
    pub fn new(level: u8, text: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            level,
            text: text.into(),
            id: id.into(),
        }
    }
}

/// A table of contents entry with nested children.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    /// Heading text.
    pub text: String,
    /// Anchor ID for linking.
    pub id: String,
    /// Heading level (1-6).
    pub level: u8,
    /// Child entries (subheadings).
    pub children: Vec<TocEntry>,
}

impl TocEntry {
    /// Create a new TOC entry from a heading.
    pub fn from_heading(heading: &Heading) -> Self {
        Self {
            text: heading.text.clone(),
            id: heading.id.clone(),
            level: heading.level,
            children: Vec::new(),
        }
    }
}

/// Table of contents.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableOfContents {
    /// Top-level entries.
    pub entries: Vec<TocEntry>,
}

impl TableOfContents {
    /// Create an empty TOC.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build TOC from a list of headings.
    pub fn from_headings(headings: &[Heading]) -> Self {
        let mut toc = Self::new();

        if headings.is_empty() {
            return toc;
        }

        // Find minimum level (available for future normalization).
        let _min_level = headings.iter().map(|h| h.level).min().unwrap_or(1);

        // Build hierarchical structure.
        let mut stack: Vec<(u8, usize)> = Vec::new(); // (level, index in parent's children)

        for heading in headings {
            let entry = TocEntry::from_heading(heading);
            let level = heading.level;

            // Pop stack until we find a parent.
            while let Some(&(parent_level, _)) = stack.last() {
                if parent_level < level {
                    break;
                }
                stack.pop();
            }

            if stack.is_empty() {
                // Top-level entry.
                toc.entries.push(entry);
                stack.push((level, toc.entries.len() - 1));
            } else {
                // Find parent and add as child.
                let parent = Self::get_entry_mut(&mut toc.entries, &stack);
                parent.children.push(entry);
                let child_idx = parent.children.len() - 1;
                stack.push((level, child_idx));
            }
        }

        toc
    }

    /// Get mutable reference to entry at stack path.
    fn get_entry_mut<'a>(entries: &'a mut [TocEntry], stack: &[(u8, usize)]) -> &'a mut TocEntry {
        let mut current = &mut entries[stack[0].1];
        for &(_, idx) in &stack[1..] {
            current = &mut current.children[idx];
        }
        current
    }

    /// Check if TOC is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get total number of entries (including nested).
    pub fn len(&self) -> usize {
        fn count(entries: &[TocEntry]) -> usize {
            entries.iter().map(|e| 1 + count(&e.children)).sum()
        }
        count(&self.entries)
    }

    /// Render TOC as HTML.
    pub fn to_html(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut html = String::new();
        html.push_str("<nav class=\"toc\">\n");
        html.push_str("<h2 class=\"toc-title\">Table of Contents</h2>\n");
        Self::render_entries(&self.entries, &mut html, 0);
        html.push_str("</nav>\n");
        html
    }

    /// Render TOC as HTML without wrapper.
    pub fn to_html_list(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut html = String::new();
        Self::render_entries(&self.entries, &mut html, 0);
        html
    }

    /// Render entries recursively.
    fn render_entries(entries: &[TocEntry], html: &mut String, depth: usize) {
        if entries.is_empty() {
            return;
        }

        let indent = "  ".repeat(depth);
        html.push_str(&format!("{}<ul class=\"toc-list toc-level-{}\">\n", indent, depth + 1));

        for entry in entries {
            html.push_str(&format!(
                "{}<li class=\"toc-item\"><a href=\"#{}\">{}</a>",
                indent, entry.id, html_escape(&entry.text)
            ));

            if !entry.children.is_empty() {
                html.push('\n');
                Self::render_entries(&entry.children, html, depth + 1);
                html.push_str(&format!("{}</li>\n", indent));
            } else {
                html.push_str("</li>\n");
            }
        }

        html.push_str(&format!("{}</ul>\n", indent));
    }

    /// Render TOC as markdown.
    pub fn to_markdown(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut md = String::new();
        Self::render_entries_md(&self.entries, &mut md, 0);
        md
    }

    /// Render entries as markdown recursively.
    fn render_entries_md(entries: &[TocEntry], md: &mut String, depth: usize) {
        for entry in entries {
            let indent = "  ".repeat(depth);
            md.push_str(&format!("{}- [{}](#{})\n", indent, entry.text, entry.id));
            Self::render_entries_md(&entry.children, md, depth + 1);
        }
    }
}

/// Extract headings from markdown content.
pub fn extract_headings(markdown: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    for caps in HEADING_RE.captures_iter(markdown) {
        let level = caps.get(1).map(|m| m.as_str().len() as u8).unwrap_or(1);
        let text = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();

        // Use custom ID if provided, otherwise generate from text.
        let id = caps.get(3)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| generate_id(&text));

        // Ensure unique ID.
        let unique_id = make_unique_id(&id, &mut seen_ids);
        seen_ids.insert(unique_id.clone());

        headings.push(Heading::new(level, strip_markdown(&text), unique_id));
    }

    headings
}

/// Extract headings from rendered HTML content.
pub fn extract_headings_html(html: &str) -> Vec<Heading> {
    let mut headings: Vec<(usize, Heading)> = Vec::new();
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Process each heading level.
    let regexes: &[(u8, &Regex)] = &[
        (1, &HTML_H1_RE), (2, &HTML_H2_RE), (3, &HTML_H3_RE),
        (4, &HTML_H4_RE), (5, &HTML_H5_RE), (6, &HTML_H6_RE),
    ];

    for &(level, re) in regexes {
        for caps in re.captures_iter(html) {
            let text = caps.get(2)
                .map(|m| strip_html(m.as_str()))
                .unwrap_or_default();

            // Use existing ID if present, otherwise generate.
            let id = caps.get(1)
                .filter(|m| !m.as_str().is_empty())
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| generate_id(&text));

            let unique_id = make_unique_id(&id, &mut seen_ids);
            seen_ids.insert(unique_id.clone());

            // Store with position in original text for sorting.
            let pos = caps.get(0).map(|m| m.start()).unwrap_or(0);
            headings.push((pos, Heading::new(level, text, unique_id)));
        }
    }

    // Sort by position in document.
    headings.sort_by_key(|(pos, _)| *pos);

    headings.into_iter().map(|(_, h)| h).collect()
}

/// Generate an ID from heading text.
pub fn generate_id(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' || c == '.' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Make an ID unique by appending a number if needed.
fn make_unique_id(id: &str, seen: &std::collections::HashSet<String>) -> String {
    if !seen.contains(id) {
        return id.to_string();
    }

    let mut counter = 1;
    loop {
        let new_id = format!("{}-{}", id, counter);
        if !seen.contains(&new_id) {
            return new_id;
        }
        counter += 1;
    }
}

/// Strip markdown formatting from text.
fn strip_markdown(text: &str) -> String {
    // Remove bold/italic markers.
    let text = text.replace("**", "").replace("__", "");
    let text = text.replace('*', "").replace('_', "");
    // Remove inline code.
    let text = text.replace('`', "");
    // Remove links [text](url) -> text.
    static LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\[([^\]]+)\]\([^)]+\)").expect("invalid regex")
    });
    LINK_RE.replace_all(&text, "$1").to_string()
}

/// Strip HTML tags from text.
fn strip_html(html: &str) -> String {
    static TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"<[^>]+>").expect("invalid regex")
    });
    let text = TAG_RE.replace_all(html, "");
    // Unescape HTML entities.
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

/// HTML-escape a string.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Add anchor IDs to headings in HTML content.
pub fn add_heading_ids(html: &str) -> String {
    let mut result = html.to_string();
    let mut seen_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Process each heading level with separate regex (backrefs not supported).
    static H1_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h1([^>]*)>(.*?)</h1>"#).expect("regex"));
    static H2_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h2([^>]*)>(.*?)</h2>"#).expect("regex"));
    static H3_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h3([^>]*)>(.*?)</h3>"#).expect("regex"));
    static H4_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h4([^>]*)>(.*?)</h4>"#).expect("regex"));
    static H5_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h5([^>]*)>(.*?)</h5>"#).expect("regex"));
    static H6_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<h6([^>]*)>(.*?)</h6>"#).expect("regex"));

    let regexes: &[(&str, &Regex)] = &[
        ("h1", &H1_TAG_RE), ("h2", &H2_TAG_RE), ("h3", &H3_TAG_RE),
        ("h4", &H4_TAG_RE), ("h5", &H5_TAG_RE), ("h6", &H6_TAG_RE),
    ];

    for &(tag, re) in regexes {
        result = re.replace_all(&result, |caps: &rustmax::regex::Captures| {
            let attrs = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let content = caps.get(2).map(|m| m.as_str()).unwrap_or("");

            // Check if already has an ID.
            if attrs.contains("id=") {
                return caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default();
            }

            // Generate ID from content.
            let text = strip_html(content);
            let id = generate_id(&text);
            let unique_id = make_unique_id(&id, &mut seen_ids);
            seen_ids.insert(unique_id.clone());

            format!("<{} id=\"{}\"{}>{}</{}>", tag, unique_id, attrs, content, tag)
        }).into_owned();
    }

    result
}

/// TOC configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocOptions {
    /// Minimum heading level to include (1-6).
    #[serde(default = "default_min_level")]
    pub min_level: u8,
    /// Maximum heading level to include (1-6).
    #[serde(default = "default_max_level")]
    pub max_level: u8,
    /// Whether to include the TOC title.
    #[serde(default = "default_include_title")]
    pub include_title: bool,
    /// Custom TOC title.
    #[serde(default = "default_toc_title")]
    pub title: String,
}

impl Default for TocOptions {
    fn default() -> Self {
        Self {
            min_level: default_min_level(),
            max_level: default_max_level(),
            include_title: default_include_title(),
            title: default_toc_title(),
        }
    }
}

fn default_min_level() -> u8 { 1 }
fn default_max_level() -> u8 { 6 }
fn default_include_title() -> bool { true }
fn default_toc_title() -> String { "Table of Contents".to_string() }

impl TocOptions {
    /// Filter headings by level.
    pub fn filter_headings(&self, headings: &[Heading]) -> Vec<Heading> {
        headings
            .iter()
            .filter(|h| h.level >= self.min_level && h.level <= self.max_level)
            .cloned()
            .collect()
    }
}

/// Generate TOC from markdown content with options.
pub fn generate_toc(markdown: &str, options: &TocOptions) -> TableOfContents {
    let headings = extract_headings(markdown);
    let filtered = options.filter_headings(&headings);
    TableOfContents::from_headings(&filtered)
}

/// Generate TOC from HTML content with options.
pub fn generate_toc_html(html: &str, options: &TocOptions) -> TableOfContents {
    let headings = extract_headings_html(html);
    let filtered = options.filter_headings(&headings);
    TableOfContents::from_headings(&filtered)
}

/// Generate CSS for TOC styling.
pub fn generate_toc_css() -> &'static str {
    r#".toc {
  background: var(--code-bg, #f5f5f5);
  border: 1px solid var(--border, #e0e0e0);
  border-radius: 6px;
  padding: 1rem 1.5rem;
  margin: 1.5rem 0;
}
.toc-title {
  font-size: 1.1rem;
  margin: 0 0 0.75rem 0;
  color: var(--fg, #333);
}
.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}
.toc-list .toc-list {
  padding-left: 1.25rem;
  margin-top: 0.25rem;
}
.toc-item {
  margin: 0.25rem 0;
  line-height: 1.4;
}
.toc-item a {
  color: var(--accent, #0066cc);
  text-decoration: none;
}
.toc-item a:hover {
  text-decoration: underline;
}
.toc-level-1 > .toc-item > a {
  font-weight: 500;
}
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_headings_basic() {
        let md = "# Title\n\nSome text.\n\n## Section 1\n\nMore text.\n\n### Subsection\n\n## Section 2\n";
        let headings = extract_headings(md);

        assert_eq!(headings.len(), 4);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].text, "Title");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[1].text, "Section 1");
        assert_eq!(headings[2].level, 3);
        assert_eq!(headings[2].text, "Subsection");
        assert_eq!(headings[3].level, 2);
        assert_eq!(headings[3].text, "Section 2");
    }

    #[test]
    fn test_extract_headings_with_custom_id() {
        let md = "# Title {#custom-id}\n\n## Section {#my-section}\n";
        let headings = extract_headings(md);

        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].id, "custom-id");
        assert_eq!(headings[1].id, "my-section");
    }

    #[test]
    fn test_extract_headings_with_formatting() {
        let md = "# **Bold** Title\n\n## Section with `code`\n\n### [Link](http://example.com)\n";
        let headings = extract_headings(md);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].text, "Bold Title");
        assert_eq!(headings[1].text, "Section with code");
        assert_eq!(headings[2].text, "Link");
    }

    #[test]
    fn test_generate_id() {
        assert_eq!(generate_id("Hello World"), "hello-world");
        assert_eq!(generate_id("Section 1.2"), "section-1-2");
        assert_eq!(generate_id("What's New?"), "whats-new");
        assert_eq!(generate_id("  Multiple   Spaces  "), "multiple-spaces");
        assert_eq!(generate_id("CamelCase"), "camelcase");
    }

    #[test]
    fn test_unique_ids() {
        let md = "# Title\n\n## Title\n\n### Title\n";
        let headings = extract_headings(md);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].id, "title");
        assert_eq!(headings[1].id, "title-1");
        assert_eq!(headings[2].id, "title-2");
    }

    #[test]
    fn test_toc_from_headings() {
        let headings = vec![
            Heading::new(1, "Title", "title"),
            Heading::new(2, "Section 1", "section-1"),
            Heading::new(3, "Subsection 1.1", "subsection-1-1"),
            Heading::new(2, "Section 2", "section-2"),
        ];

        let toc = TableOfContents::from_headings(&headings);

        assert_eq!(toc.entries.len(), 1);
        assert_eq!(toc.entries[0].text, "Title");
        assert_eq!(toc.entries[0].children.len(), 2);
        assert_eq!(toc.entries[0].children[0].text, "Section 1");
        assert_eq!(toc.entries[0].children[0].children.len(), 1);
        assert_eq!(toc.entries[0].children[1].text, "Section 2");
    }

    #[test]
    fn test_toc_flat_headings() {
        let headings = vec![
            Heading::new(2, "A", "a"),
            Heading::new(2, "B", "b"),
            Heading::new(2, "C", "c"),
        ];

        let toc = TableOfContents::from_headings(&headings);

        assert_eq!(toc.entries.len(), 3);
        assert_eq!(toc.entries[0].children.len(), 0);
    }

    #[test]
    fn test_toc_to_html() {
        let headings = vec![
            Heading::new(1, "Title", "title"),
            Heading::new(2, "Section", "section"),
        ];

        let toc = TableOfContents::from_headings(&headings);
        let html = toc.to_html();

        assert!(html.contains("<nav class=\"toc\">"));
        assert!(html.contains("<h2 class=\"toc-title\">Table of Contents</h2>"));
        assert!(html.contains("<a href=\"#title\">Title</a>"));
        assert!(html.contains("<a href=\"#section\">Section</a>"));
    }

    #[test]
    fn test_toc_to_markdown() {
        let headings = vec![
            Heading::new(1, "Title", "title"),
            Heading::new(2, "Section", "section"),
        ];

        let toc = TableOfContents::from_headings(&headings);
        let md = toc.to_markdown();

        assert!(md.contains("- [Title](#title)"));
        assert!(md.contains("  - [Section](#section)"));
    }

    #[test]
    fn test_toc_len() {
        let headings = vec![
            Heading::new(1, "Title", "title"),
            Heading::new(2, "Section 1", "section-1"),
            Heading::new(3, "Sub", "sub"),
            Heading::new(2, "Section 2", "section-2"),
        ];

        let toc = TableOfContents::from_headings(&headings);

        assert_eq!(toc.len(), 4);
    }

    #[test]
    fn test_toc_empty() {
        let toc = TableOfContents::new();
        assert!(toc.is_empty());
        assert_eq!(toc.len(), 0);
        assert_eq!(toc.to_html(), "");
    }

    #[test]
    fn test_add_heading_ids() {
        let html = "<h1>Title</h1><p>Text</p><h2>Section</h2>";
        let result = add_heading_ids(html);

        assert!(result.contains("<h1 id=\"title\">Title</h1>"));
        assert!(result.contains("<h2 id=\"section\">Section</h2>"));
    }

    #[test]
    fn test_add_heading_ids_preserves_existing() {
        let html = "<h1 id=\"custom\">Title</h1><h2>Section</h2>";
        let result = add_heading_ids(html);

        assert!(result.contains("<h1 id=\"custom\">Title</h1>"));
        assert!(result.contains("<h2 id=\"section\">Section</h2>"));
    }

    #[test]
    fn test_extract_headings_html() {
        let html = "<h1 id=\"title\">Title</h1><p>Text</p><h2>Section</h2>";
        let headings = extract_headings_html(html);

        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].text, "Title");
        assert_eq!(headings[0].id, "title");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[1].text, "Section");
    }

    #[test]
    fn test_toc_options_filter() {
        let headings = vec![
            Heading::new(1, "H1", "h1"),
            Heading::new(2, "H2", "h2"),
            Heading::new(3, "H3", "h3"),
            Heading::new(4, "H4", "h4"),
        ];

        let options = TocOptions {
            min_level: 2,
            max_level: 3,
            ..Default::default()
        };

        let filtered = options.filter_headings(&headings);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].text, "H2");
        assert_eq!(filtered[1].text, "H3");
    }

    #[test]
    fn test_generate_toc() {
        let md = "# Title\n\n## Section 1\n\n### Sub\n\n## Section 2\n";
        let options = TocOptions::default();
        let toc = generate_toc(md, &options);

        assert_eq!(toc.len(), 4);
    }

    #[test]
    fn test_toc_css() {
        let css = generate_toc_css();
        assert!(css.contains(".toc"));
        assert!(css.contains(".toc-list"));
        assert!(css.contains(".toc-item"));
    }

    #[test]
    fn test_strip_markdown() {
        assert_eq!(strip_markdown("**bold**"), "bold");
        assert_eq!(strip_markdown("*italic*"), "italic");
        assert_eq!(strip_markdown("`code`"), "code");
        assert_eq!(strip_markdown("[link](http://x.com)"), "link");
    }

    #[test]
    fn test_strip_html() {
        assert_eq!(strip_html("<strong>bold</strong>"), "bold");
        assert_eq!(strip_html("<a href=\"#\">link</a>"), "link");
        assert_eq!(strip_html("a &amp; b"), "a & b");
    }

    #[test]
    fn test_deep_nesting() {
        let headings = vec![
            Heading::new(1, "L1", "l1"),
            Heading::new(2, "L2", "l2"),
            Heading::new(3, "L3", "l3"),
            Heading::new(4, "L4", "l4"),
            Heading::new(5, "L5", "l5"),
            Heading::new(6, "L6", "l6"),
        ];

        let toc = TableOfContents::from_headings(&headings);

        assert_eq!(toc.len(), 6);
        assert_eq!(toc.entries.len(), 1);
        assert_eq!(toc.entries[0].children.len(), 1);
    }

    #[test]
    fn test_skipped_levels() {
        let headings = vec![
            Heading::new(1, "Title", "title"),
            Heading::new(3, "Jump to H3", "h3"),
            Heading::new(2, "Back to H2", "h2"),
        ];

        let toc = TableOfContents::from_headings(&headings);

        assert_eq!(toc.entries.len(), 1);
        assert_eq!(toc.entries[0].children.len(), 2);
    }
}
