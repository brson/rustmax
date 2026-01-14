//! Simple mdbook-compatible renderer producing basic HTML without scripting.
//!
//! This module provides a minimal alternative to mdbook that renders books
//! to simple, static HTML with minimal styling. It parses the standard
//! mdbook SUMMARY.md format and renders markdown to HTML.

use rmx::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use pulldown_cmark::{Parser, Options, html};

/// A chapter in the book.
#[derive(Debug, Clone)]
pub struct Chapter {
    pub title: String,
    pub path: Option<PathBuf>,
    pub children: Vec<Chapter>,
}

/// Parsed book structure.
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

/// Build a book from input directory to output directory.
pub fn build(input: &Path, output: &Path) -> AnyResult<()> {
    // Parse book.toml for title.
    let title = parse_book_title(input)?;

    // Find the source directory.
    let src_dir = find_src_dir(input)?;

    // Parse SUMMARY.md.
    let summary_path = src_dir.join("SUMMARY.md");
    if !summary_path.exists() {
        return Err(anyhow!("SUMMARY.md not found at {}", summary_path.display()));
    }
    let summary_content = fs::read_to_string(&summary_path)?;
    let chapters = parse_summary(&summary_content)?;

    let book = Book { title, chapters };

    // Create output directory.
    fs::create_dir_all(output)?;

    // Copy shared theme CSS from www/.
    let themes_css = include_str!("../../../www/rustmax-themes.css");
    fs::write(output.join("rustmax-themes.css"), themes_css)?;

    // Generate book-specific CSS.
    let css = generate_css();
    fs::write(output.join("rmxbook.css"), &css)?;

    // Render each chapter.
    render_book(&book, &src_dir, output)?;

    // Generate index.html redirect to first chapter.
    generate_index(&book, output)?;

    Ok(())
}

fn parse_book_title(input: &Path) -> AnyResult<String> {
    let book_toml = input.join("book.toml");
    if book_toml.exists() {
        let content = fs::read_to_string(&book_toml)?;
        if let Ok(toml_value) = toml::from_str::<toml::Value>(&content) {
            if let Some(title) = toml_value
                .get("book")
                .and_then(|b| b.get("title"))
                .and_then(|t| t.as_str())
            {
                return Ok(title.to_string());
            }
        }
    }
    // Default title if book.toml not found or doesn't have title.
    Ok("Book".to_string())
}

fn find_src_dir(input: &Path) -> AnyResult<PathBuf> {
    // Check book.toml for src path.
    let book_toml = input.join("book.toml");
    if book_toml.exists() {
        let content = fs::read_to_string(&book_toml)?;
        if let Ok(toml_value) = toml::from_str::<toml::Value>(&content) {
            if let Some(src) = toml_value
                .get("book")
                .and_then(|b| b.get("src"))
                .and_then(|s| s.as_str())
            {
                let src_path = input.join(src);
                if src_path.exists() {
                    return Ok(src_path);
                }
            }
        }
    }
    // Default to "src" subdirectory.
    let src_path = input.join("src");
    if src_path.exists() {
        return Ok(src_path);
    }
    // Fall back to input directory itself.
    Ok(input.to_path_buf())
}

fn parse_summary(content: &str) -> AnyResult<Vec<Chapter>> {
    let mut chapters = Vec::new();
    let mut current_indent = 0;
    let mut stack: Vec<(usize, Vec<Chapter>)> = vec![(0, Vec::new())];

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and separators.
        if trimmed.is_empty() || trimmed == "---" || trimmed.starts_with('#') {
            continue;
        }

        // Parse markdown link: [Title](path.md)
        if let Some(chapter) = parse_chapter_link(line) {
            let indent = line.len() - line.trim_start().len();
            let level = indent / 2; // Assume 2-space indentation.

            // Pop stack until we find the right level.
            while stack.len() > level + 1 {
                let (_, children) = stack.pop().unwrap();
                if let Some((_, parent_children)) = stack.last_mut() {
                    if let Some(last) = parent_children.last_mut() {
                        last.children = children;
                    }
                }
            }

            // Push new level if needed.
            if level >= stack.len() {
                stack.push((level, Vec::new()));
            }

            if let Some((_, children)) = stack.last_mut() {
                children.push(chapter);
            }

            current_indent = level;
        }
    }

    // Flatten stack.
    while stack.len() > 1 {
        let (_, children) = stack.pop().unwrap();
        if let Some((_, parent_children)) = stack.last_mut() {
            if let Some(last) = parent_children.last_mut() {
                last.children = children;
            }
        }
    }

    if let Some((_, root_children)) = stack.pop() {
        chapters = root_children;
    }

    Ok(chapters)
}

fn parse_chapter_link(line: &str) -> Option<Chapter> {
    let trimmed = line.trim().trim_start_matches('-').trim();

    // Match [Title](path)
    let start = trimmed.find('[')?;
    let mid = trimmed.find("](")?;
    let end = trimmed.rfind(')')?;

    if start >= mid || mid >= end {
        return None;
    }

    let title = trimmed[start + 1..mid].to_string();
    let path_str = &trimmed[mid + 2..end];

    let path = if path_str.is_empty() || path_str == "#" {
        None
    } else {
        Some(PathBuf::from(path_str))
    };

    Some(Chapter {
        title,
        path,
        children: Vec::new(),
    })
}

fn render_book(book: &Book, src_dir: &Path, output: &Path) -> AnyResult<()> {
    // Flatten chapters for navigation.
    let flat_chapters = flatten_chapters(&book.chapters);

    for (i, chapter) in flat_chapters.iter().enumerate() {
        if let Some(ref path) = chapter.path {
            let prev = if i > 0 { flat_chapters.get(i - 1) } else { None };
            let next = flat_chapters.get(i + 1);

            render_chapter(book, chapter, prev.copied(), next.copied(), &book.chapters, src_dir, output)?;
        }
    }

    Ok(())
}

fn flatten_chapters(chapters: &[Chapter]) -> Vec<&Chapter> {
    let mut result = Vec::new();
    for chapter in chapters {
        result.push(chapter);
        result.extend(flatten_chapters(&chapter.children));
    }
    result
}

fn render_chapter(
    book: &Book,
    chapter: &Chapter,
    prev: Option<&Chapter>,
    next: Option<&Chapter>,
    all_chapters: &[Chapter],
    src_dir: &Path,
    output: &Path,
) -> AnyResult<()> {
    let Some(ref rel_path) = chapter.path else {
        return Ok(());
    };

    let md_path = src_dir.join(rel_path);
    if !md_path.exists() {
        eprintln!("  Warning: {} not found", md_path.display());
        return Ok(());
    }

    let content = fs::read_to_string(&md_path)?;
    let html_content = markdown_to_html(&content);

    // Calculate relative path to root for CSS.
    // Filter out CurDir (.) components when counting depth.
    let depth = rel_path
        .components()
        .filter(|c| !matches!(c, std::path::Component::CurDir))
        .count()
        .saturating_sub(1);
    let path_to_root = if depth == 0 {
        String::new()
    } else {
        "../".repeat(depth)
    };

    // Build navigation.
    let nav_html = build_nav(all_chapters, Some(rel_path), &path_to_root);

    // Build prev/next links.
    let prev_link = prev
        .and_then(|p| p.path.as_ref())
        .map(|p| {
            let href = format!("{}{}", path_to_root, p.with_extension("html").display());
            format!(r#"<a href="{}" class="prev">Previous</a>"#, href)
        })
        .unwrap_or_default();

    let next_link = next
        .and_then(|n| n.path.as_ref())
        .map(|p| {
            let href = format!("{}{}", path_to_root, p.with_extension("html").display());
            format!(r#"<a href="{}" class="next">Next</a>"#, href)
        })
        .unwrap_or_default();

    let page_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title} - {book_title}</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,400;0,700;1,400;1,700&family=Source+Serif+4:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="{path_to_root}rustmax-themes.css">
    <link rel="stylesheet" href="{path_to_root}rmxbook.css">
    <script>{script}</script>
</head>
<body>
    <button class="nav-toggle" aria-label="Toggle navigation"></button>
    <nav class="sidebar">
        <div class="sidebar-title"><a href="{path_to_root}index.html">{book_title}</a></div>
        {nav}
    </nav>
    <main>
        <article>
            {content}
        </article>
        <div class="nav-links">
            {prev}
            {next}
        </div>
    </main>
</body>
</html>"#,
        title = html_escape(&chapter.title),
        book_title = html_escape(&book.title),
        path_to_root = path_to_root,
        nav = nav_html,
        content = html_content,
        prev = prev_link,
        next = next_link,
        script = generate_script(),
    );

    // Write HTML file.
    let html_path = rel_path.with_extension("html");
    let out_path = output.join(&html_path);

    // Create parent directories.
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&out_path, page_html)?;

    Ok(())
}

fn build_nav(chapters: &[Chapter], current: Option<&PathBuf>, path_to_root: &str) -> String {
    let mut html = String::from("<ul>\n");

    for chapter in chapters {
        let is_current = current.map_or(false, |c| chapter.path.as_ref() == Some(c));
        let class = if is_current { r#" class="current""# } else { "" };

        if let Some(ref path) = chapter.path {
            let href = format!("{}{}", path_to_root, path.with_extension("html").display());
            html.push_str(&format!(
                r#"<li{}><a href="{}">{}</a>"#,
                class,
                href,
                html_escape(&chapter.title)
            ));
        } else {
            html.push_str(&format!("<li{}>{}", class, html_escape(&chapter.title)));
        }

        if !chapter.children.is_empty() {
            html.push_str(&build_nav(&chapter.children, current, path_to_root));
        }

        html.push_str("</li>\n");
    }

    html.push_str("</ul>\n");
    html
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn generate_index(book: &Book, output: &Path) -> AnyResult<()> {
    // Find first chapter with a path.
    let first_chapter = flatten_chapters(&book.chapters)
        .into_iter()
        .find(|c| c.path.is_some());

    let Some(redirect_target) = first_chapter
        .and_then(|c| c.path.as_ref())
        .map(|p| p.with_extension("html").display().to_string())
    else {
        return Err(anyhow!("No chapters found to redirect index.html to"));
    };

    let index_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0; url={target}">
    <title>{title}</title>
    <link rel="stylesheet" href="rmxbook.css">
</head>
<body>
    <p>Redirecting to <a href="{target}">{target}</a>...</p>
</body>
</html>"#,
        title = html_escape(&book.title),
        target = redirect_target,
    );

    fs::write(output.join("index.html"), index_html)?;

    Ok(())
}

fn generate_script() -> &'static str {
    r#"(function(){
  // Nav toggle.
  var k = 'rmxbook-nav';
  var stored = localStorage.getItem(k);
  var mobile = window.matchMedia('(max-width: 768px)');
  var collapsed = stored ? stored === 'collapsed' : mobile.matches;
  if (collapsed) document.documentElement.classList.add('nav-collapsed');
  document.addEventListener('click', function(e) {
    if (e.target.classList.contains('nav-toggle')) {
      var c = document.documentElement.classList.toggle('nav-collapsed');
      localStorage.setItem(k, c ? 'collapsed' : 'expanded');
    }
  });
})();"#
}

fn generate_css() -> String {
    r#"/* rmxbook - uses shared rustmax-themes.css variables */
:root {
    --rmx-sidebar-width: 280px;
    --rmx-toggle-size: 36px;
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    padding: 0;
    font: var(--rmx-font-text);
    line-height: 1.6;
    background: var(--rmx-color-bg);
    color: var(--rmx-color-fg);
}

body {
    display: flex;
    min-height: 100vh;
}

a { color: var(--rmx-color-links); text-decoration: none; }
a:hover { color: var(--rmx-color-accents); }

/* Toggle - sticky, stays visible when scrolling */
.nav-toggle {
    position: sticky;
    top: 0.5rem;
    align-self: flex-start;
    flex-shrink: 0;
    width: var(--rmx-toggle-size);
    height: var(--rmx-toggle-size);
    margin: 0.5rem;
    padding: 0;
    border: 1px dashed var(--rmx-color-border);
    background: var(--rmx-color-bg-alt);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
}

.nav-toggle::before {
    content: '';
    width: 18px;
    height: 2px;
    background: var(--rmx-color-fg);
    box-shadow: 0 6px 0 var(--rmx-color-fg), 0 -6px 0 var(--rmx-color-fg);
}

.nav-toggle:hover { background: var(--rmx-color-bg); }

/* Sidebar - in flow, animates width to collapse */
.sidebar {
    flex-shrink: 0;
    width: var(--rmx-sidebar-width);
    height: 100vh;
    position: sticky;
    top: 0;
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--rmx-color-bg-alt);
    border-right: 1px dashed var(--rmx-color-border);
    padding: 1rem;
    transition: width 0.2s ease, padding 0.2s ease, border 0.2s ease;
}

.nav-collapsed .sidebar {
    width: 0;
    padding-left: 0;
    padding-right: 0;
    border-right-color: transparent;
}

.sidebar-title {
    font: var(--rmx-font-em);
    font-size: 1.2rem;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px dashed var(--rmx-color-border);
    white-space: nowrap;
}

.sidebar-title a { color: var(--rmx-color-fg); }
.sidebar-title a:hover { color: var(--rmx-color-accents); }
.sidebar ul { list-style: none; padding: 0; margin: 0; }
.sidebar li { margin: 0.3rem 0; white-space: nowrap; }
.sidebar li ul { padding-left: 1rem; }
.sidebar .current > a { font-weight: bold; }

/* Main - always centered with max-width */
main {
    flex: 1;
    min-width: 0;
    max-width: 50rem;
    margin: 0 auto;
    padding: 2rem 3rem;
}

article { line-height: 1.7; }

h1, h2, h3, h4, h5, h6 {
    font: var(--rmx-font-em);
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    line-height: 1.3;
}

h1 { font-size: 2rem; }
h2 { font-size: 1.5rem; }
h3 { font-size: 1.25rem; }

code {
    font: var(--rmx-font-code);
    font-size: 0.9em;
    background: var(--rmx-color-bg-alt);
    padding: 0.15em 0.3em;
}

pre {
    background: var(--rmx-color-bg-alt);
    padding: 1rem;
    overflow-x: auto;
}

pre code { background: none; padding: 0; }

blockquote {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 4px dashed var(--rmx-color-border);
    background: var(--rmx-color-bg-alt);
}

table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
th, td { border: 1px dashed var(--rmx-color-border); padding: 0.5rem; text-align: left; }
th { background: var(--rmx-color-bg-alt); }

.nav-links {
    display: flex;
    justify-content: space-between;
    margin-top: 3rem;
    padding-top: 1rem;
    border-top: 1px dashed var(--rmx-color-border);
}

.nav-links a {
    padding: 0.5rem 1rem;
    border: 1px dashed var(--rmx-color-border);
}

.nav-links a:hover { background: var(--rmx-color-bg-alt); }

/* Mobile: vertical stacking */
@media (max-width: 768px) {
    body { flex-direction: column; }

    .nav-toggle {
        position: relative;
        top: 0;
        align-self: stretch;
        width: 100%;
        height: var(--rmx-toggle-size);
        margin: 0;
        border-left: none;
        border-right: none;
        border-top: none;
    }

    .sidebar {
        position: relative;
        width: 100%;
        height: auto;
        max-height: 60vh;
        border-right: none;
        border-bottom: 1px dashed var(--rmx-color-border);
        transition: max-height 0.2s ease, padding 0.2s ease, border 0.2s ease;
    }

    .nav-collapsed .sidebar {
        width: 100%;
        max-height: 0;
        padding-top: 0;
        padding-bottom: 0;
        border-bottom-color: transparent;
        overflow: hidden;
    }

    .sidebar li { white-space: normal; }

    main { padding: 1rem; }
}
"#.to_string()
}
