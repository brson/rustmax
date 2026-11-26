//! Simple mdbook-compatible renderer producing basic HTML without scripting.
//!
//! This module provides a minimal alternative to mdbook that renders books
//! to simple, static HTML with minimal styling. It parses the standard
//! mdbook SUMMARY.md format and renders markdown to HTML.
//!
//! Beyond plain markdown it implements the parts of mdbook that books in the
//! Rustmax library depend on: `{{#include}}` and friends, hidden lines in
//! code blocks, `README.md` chapters becoming `index.html`, heading
//! attributes, GitHub-style alerts, and two third-party preprocessors,
//! the Rust Reference's `spec` and The Rust Programming Language's
//! `trpl-listing`.

use rmx::prelude::*;
use rmx::regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, LazyLock};

use crate::highlight::{Highlighter, html_escape};

mod include;
mod listing;
mod spec;

use spec::Spec;

/// What kind of entry a SUMMARY.md line produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChapterKind {
    /// A chapter, or a draft chapter if it has no path.
    Page,
    /// A `# Heading` naming the part that the following chapters belong to.
    PartTitle,
    /// A `---` rule between groups of chapters.
    Separator,
}

/// A chapter in the book.
#[derive(Debug, Clone)]
pub struct Chapter {
    pub kind: ChapterKind,
    pub title: String,
    pub path: Option<PathBuf>,
    /// The section number, like `1.2.`, for chapters in the numbered list.
    pub number: Option<String>,
    pub children: Vec<Chapter>,
}

/// Parsed book structure.
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

/// Where a book sits in the Rustmax library.
///
/// A book rendered on its own has none of this.
pub struct LibraryBook {
    pub name: String,
    /// The book's home on the web, with a trailing slash.
    pub upstream_url: String,
    /// The pinned source the book was rendered from.
    pub source_url: String,
    pub commit: String,
    /// Every book in the library as `(upstream_url, slug)`, used to point
    /// links between books at our copies.
    pub books: Vec<(String, String)>,
}

/// Settings read from `book.toml`.
struct Config {
    title: String,
    src_dir: PathBuf,
    /// Uses the Rust Reference's `spec` preprocessor.
    spec: bool,
    /// Uses The Rust Programming Language's `trpl-listing` preprocessor.
    listings: bool,
    smart_punctuation: bool,
    section_labels: bool,
    /// Language to the prefix that hides a line in that language's code
    /// blocks, beyond Rust's `#`.
    hidelines: BTreeMap<String, String>,
    /// Old page paths to where they moved, from `[output.html.redirect]`.
    redirects: Vec<(String, String)>,
}

/// Build a book from input directory to output directory.
pub fn build(input: &Path, output: &Path, library: Option<&LibraryBook>) -> AnyResult<()> {
    let config = load_config(input)?;
    let src_dir = &config.src_dir;

    let summary_path = src_dir.join("SUMMARY.md");
    if !summary_path.exists() {
        return Err(anyhow!("SUMMARY.md not found at {}", summary_path.display()));
    }
    let summary_content = fs::read_to_string(&summary_path)?;
    let chapters = parse_summary(&summary_content);

    let book = Book { title: config.title.clone(), chapters };

    // Collect the Rust Reference's rule identifiers, which can be linked to
    // from any page and so have to be known before rendering starts.
    let spec = config.spec.then(|| {
        Spec::collect(&book, |chapter| {
            let path = chapter.path.as_ref()?;
            fs::read_to_string(src_dir.join(path)).ok()
        })
    });

    fs::create_dir_all(output)?;
    copy_assets(src_dir, src_dir, output)?;

    fs::write(output.join("rustmax-themes.css"), include_str!("../assets/rustmax-themes.css"))?;
    fs::write(output.join("rustmax-syntax.css"), include_str!("../assets/rustmax-syntax.css"))?;
    fs::write(output.join("rmxbook.css"), generate_css(&config))?;

    render_book(&book, &config, output, spec.as_ref(), library)?;
    generate_index(&book, output)?;
    generate_redirects(&config, output, library)?;

    Ok(())
}

fn load_config(input: &Path) -> AnyResult<Config> {
    let book_toml = input.join("book.toml");
    let toml_text = if book_toml.exists() {
        fs::read_to_string(&book_toml)?
    } else {
        String::new()
    };
    let toml_value: toml::Value = toml::from_str(&toml_text)
        .with_context(|| format!("parsing {}", book_toml.display()))?;

    let get = |path: &[&str]| -> Option<&toml::Value> {
        path.iter().try_fold(&toml_value, |value, key| value.get(key))
    };

    let title = get(&["book", "title"])
        .and_then(|t| t.as_str())
        .unwrap_or("Book")
        .to_string();

    let src_dir = match get(&["book", "src"]).and_then(|s| s.as_str()) {
        Some(src) if input.join(src).exists() => input.join(src),
        _ if input.join("src").exists() => input.join("src"),
        _ => input.to_path_buf(),
    };

    let html_flag = |key: &str| get(&["output", "html", key]).and_then(|v| v.as_bool());

    let hidelines = get(&["output", "html", "code", "hidelines"])
        .and_then(|h| h.as_table())
        .map(|table| {
            table
                .iter()
                .filter_map(|(lang, prefix)| Some((lang.clone(), prefix.as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default();

    let redirects = get(&["output", "html", "redirect"])
        .and_then(|r| r.as_table())
        .map(|table| {
            table
                .iter()
                .filter_map(|(from, to)| Some((from.clone(), to.as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        title,
        src_dir,
        spec: spec::is_spec_book(&toml_text),
        listings: listing::is_listing_book(&toml_value),
        // mdbook 0.5 turns this on unless the book opts out.
        smart_punctuation: html_flag("smart-punctuation").unwrap_or(true),
        section_labels: !html_flag("no-section-label").unwrap_or(false),
        hidelines,
        redirects,
    })
}

/// Copy everything in the source directory that isn't a chapter source:
/// images, diagrams, and whatever else the chapters link to.
fn copy_assets(dir: &Path, src_root: &Path, output: &Path) -> AnyResult<()> {
    let output_abs = fs::canonicalize(output)?;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        if entry.file_type()?.is_dir() {
            // A book whose sources are its root directory may render into a
            // subdirectory of itself.
            if fs::canonicalize(&path)? == output_abs {
                continue;
            }
            copy_assets(&path, src_root, output)?;
        } else if path.extension().is_none_or(|ext| ext != "md") {
            let rel = path.strip_prefix(src_root).expect("walking below the source root");
            let dest = output.join(rel);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}

/// Parse SUMMARY.md the way mdbook does.
///
/// Links outside a list are unnumbered prefix or suffix chapters. List items
/// are numbered chapters, nested by indentation of any width. A heading
/// before the first chapter is the summary's own title and is dropped; later
/// headings are part titles. Numbering runs on across parts.
fn parse_summary(content: &str) -> Vec<Chapter> {
    let mut items = Vec::new();
    // The open levels of the numbered list, by indentation.
    let mut stack: Vec<(usize, Vec<Chapter>)> = Vec::new();
    let mut next_number = 0;
    let mut seen_chapter = false;

    fn close_list(
        stack: &mut Vec<(usize, Vec<Chapter>)>,
        items: &mut Vec<Chapter>,
        next_number: &mut u32,
    ) {
        while stack.len() > 1 {
            close_level(stack);
        }
        if let Some((_, mut chapters)) = stack.pop() {
            for chapter in &mut chapters {
                *next_number += 1;
                assign_numbers(chapter, format!("{next_number}."));
            }
            items.extend(chapters);
        }
    }

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('#') {
            close_list(&mut stack, &mut items, &mut next_number);
            if seen_chapter {
                items.push(Chapter {
                    kind: ChapterKind::PartTitle,
                    title: trimmed.trim_start_matches('#').trim().to_string(),
                    path: None,
                    number: None,
                    children: Vec::new(),
                });
            }
            continue;
        }

        if trimmed.len() >= 3 && trimmed.chars().all(|c| c == '-') {
            close_list(&mut stack, &mut items, &mut next_number);
            items.push(Chapter {
                kind: ChapterKind::Separator,
                title: String::new(),
                path: None,
                number: None,
                children: Vec::new(),
            });
            continue;
        }

        let Some(chapter) = parse_chapter_link(line) else {
            continue;
        };
        seen_chapter = true;

        let is_list_item = trimmed.starts_with('-') || trimmed.starts_with('*');
        if !is_list_item {
            close_list(&mut stack, &mut items, &mut next_number);
            items.push(chapter);
            continue;
        }

        let indent = line
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| if c == '\t' { 4 } else { 1 })
            .sum();
        while stack.len() > 1 && indent < stack.last().expect("nonempty").0 {
            close_level(&mut stack);
        }
        match stack.last() {
            Some((top, _)) if indent > *top => stack.push((indent, Vec::new())),
            Some(_) => {}
            None => stack.push((indent, Vec::new())),
        }
        stack.last_mut().expect("nonempty").1.push(chapter);
    }
    close_list(&mut stack, &mut items, &mut next_number);

    items
}

/// Pop the innermost list level, making it the children of its parent.
fn close_level(stack: &mut Vec<(usize, Vec<Chapter>)>) {
    let (_, children) = stack.pop().expect("a level to close");
    let parent = &mut stack.last_mut().expect("a parent level").1;
    match parent.last_mut() {
        Some(last) => last.children = children,
        None => parent.extend(children),
    }
}

fn assign_numbers(chapter: &mut Chapter, number: String) {
    for (i, child) in chapter.children.iter_mut().enumerate() {
        assign_numbers(child, format!("{number}{}.", i + 1));
    }
    chapter.number = Some(number);
}

fn parse_chapter_link(line: &str) -> Option<Chapter> {
    let trimmed = line.trim().trim_start_matches(['-', '*']).trim();

    // Match [Title](path). The path ends at the first `)` after the `](`,
    // not the last one on the line, which would swallow any note the entry
    // carries after the link.
    let start = trimmed.find('[')?;
    let mid = trimmed.find("](")?;
    let end = mid + 2 + trimmed[mid + 2..].find(')')?;

    if start >= mid || mid >= end {
        return None;
    }

    let title = trimmed[start + 1..mid].to_string();
    let path_str = trimmed[mid + 2..end].trim();

    let path = if path_str.is_empty() || path_str == "#" {
        None
    } else {
        Some(PathBuf::from(path_str))
    };

    Some(Chapter {
        kind: ChapterKind::Page,
        title,
        path,
        number: None,
        children: Vec::new(),
    })
}

/// The page a chapter source renders to.
///
/// mdbook renders `README.md` as the directory's `index.html`.
pub(crate) fn html_path(md_path: &Path) -> PathBuf {
    let is_readme = md_path
        .file_stem()
        .is_some_and(|stem| stem.eq_ignore_ascii_case("readme"));
    if is_readme {
        md_path.with_file_name("index.html")
    } else {
        md_path.with_extension("html")
    }
}

/// `path` with `/` separators and no `./` components, for use in URLs.
pub(crate) fn url_path(path: &Path) -> String {
    path.components()
        .filter(|c| !matches!(c, Component::CurDir))
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn render_book(
    book: &Book,
    config: &Config,
    output: &Path,
    spec: Option<&Spec>,
    library: Option<&LibraryBook>,
) -> AnyResult<()> {
    let pages: Vec<&Chapter> = flatten_chapters(&book.chapters)
        .into_iter()
        .filter(|c| c.path.is_some())
        .collect();

    for (i, chapter) in pages.iter().enumerate() {
        let prev = i.checked_sub(1).map(|i| pages[i]);
        let next = pages.get(i + 1).copied();
        render_chapter(book, config, chapter, prev, next, output, spec, library)?;
    }

    Ok(())
}

pub(crate) fn flatten_chapters(chapters: &[Chapter]) -> Vec<&Chapter> {
    let mut result = Vec::new();
    for chapter in chapters {
        result.push(chapter);
        result.extend(flatten_chapters(&chapter.children));
    }
    result
}

#[allow(clippy::too_many_arguments)]
fn render_chapter(
    book: &Book,
    config: &Config,
    chapter: &Chapter,
    prev: Option<&Chapter>,
    next: Option<&Chapter>,
    output: &Path,
    spec: Option<&Spec>,
    library: Option<&LibraryBook>,
) -> AnyResult<()> {
    let rel_path = chapter.path.as_ref().expect("only chapters with paths are rendered");

    let md_path = config.src_dir.join(rel_path);
    if !md_path.exists() {
        eprintln!("  Warning: {} not found", md_path.display());
        return Ok(());
    }

    let page = html_path(rel_path);
    let page_url = url_path(&page);
    let depth = page_url.matches('/').count();
    let path_to_root = "../".repeat(depth);

    let content = fs::read_to_string(&md_path)?;
    let chapter_dir = md_path.parent().expect("a file has a parent");
    let (content, title_override) = include::expand(&content, chapter_dir);
    let content = if config.listings {
        listing::rewrite(&content, inline_markdown)
    } else {
        content
    };
    let content = match spec {
        Some(spec) => spec.preprocess(&content, &path_to_root),
        None => content,
    };

    let page_ctx = PageContext {
        config,
        page_url: &page_url,
        depth,
        spec: spec.map(|spec| (spec, path_to_root.as_str())),
        library,
    };
    let html_content = markdown_to_html(&content, &page_ctx);
    let mermaid = html_content.contains("<pre class=\"mermaid\">");

    let nav_html = build_nav(&book.chapters, rel_path, &path_to_root, config.section_labels);

    let nav_link = |chapter: Option<&Chapter>, class: &str, label: &str| {
        chapter
            .and_then(|c| c.path.as_ref())
            .map(|p| {
                format!(
                    r#"<a href="{path_to_root}{}" class="{class}" rel="{class}">{label}</a>"#,
                    url_path(&html_path(p))
                )
            })
            .unwrap_or_default()
    };
    let prev_link = nav_link(prev, "prev", "Previous");
    let next_link = nav_link(next, "next", "Next");

    let title = title_override.unwrap_or_else(|| strip_inline_markdown(&chapter.title));
    let footer = library.map(|lib| provenance_footer(lib, &page_url)).unwrap_or_default();
    let mermaid_script = if mermaid { MERMAID_SCRIPT } else { "" };

    let page_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title} - {book_title}</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,400;0,700;1,400;1,700&family=Zilla+Slab:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="{path_to_root}rustmax-themes.css">
    <link rel="stylesheet" href="{path_to_root}rustmax-syntax.css">
    <link rel="stylesheet" href="{path_to_root}rmxbook.css">
    <script>{script}</script>{mermaid_script}
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
        {footer}
    </main>
</body>
</html>"#,
        title = html_escape(&title),
        book_title = html_escape(&book.title),
        nav = nav_html,
        content = html_content,
        prev = prev_link,
        next = next_link,
        script = generate_script(),
    );

    let out_path = output.join(&page);
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out_path, page_html)?;

    Ok(())
}

/// Say where a library page comes from, and link to the original.
fn provenance_footer(lib: &LibraryBook, page_url: &str) -> String {
    let short_commit = &lib.commit[..lib.commit.len().min(10)];
    format!(
        r#"<footer class="provenance">
            A copy of <a href="{upstream}">{name}</a>,
            rendered for the <a href="{up}../../library.html">Rustmax library</a>
            from <a href="{source}">{short_commit}</a>.
            <a href="{upstream}{page_url}">Read this page upstream</a>.
        </footer>"#,
        upstream = html_escape(&lib.upstream_url),
        name = html_escape(&lib.name),
        up = "../".repeat(page_url.matches('/').count()),
        source = html_escape(&lib.source_url),
    )
}

fn build_nav(
    chapters: &[Chapter],
    current: &Path,
    path_to_root: &str,
    section_labels: bool,
) -> String {
    let mut html = String::from("<ul>\n");

    for chapter in chapters {
        match chapter.kind {
            ChapterKind::PartTitle => {
                html.push_str(&format!(
                    "<li class=\"part-title\">{}</li>\n",
                    inline_markdown(&chapter.title)
                ));
                continue;
            }
            ChapterKind::Separator => {
                html.push_str("<li class=\"separator\"></li>\n");
                continue;
            }
            ChapterKind::Page => {}
        }

        let is_current = chapter.path.as_deref() == Some(current);
        let class = if is_current { r#" class="current""# } else { "" };
        let number = match (&chapter.number, section_labels) {
            (Some(number), true) => format!("<span class=\"section-number\">{number}</span> "),
            _ => String::new(),
        };
        let title = inline_markdown(&chapter.title);

        match &chapter.path {
            Some(path) => html.push_str(&format!(
                r#"<li{class}><a href="{path_to_root}{}">{number}{title}</a>"#,
                url_path(&html_path(path)),
            )),
            None => html.push_str(&format!(r#"<li class="draft">{number}{title}"#)),
        }

        if !chapter.children.is_empty() {
            html.push_str(&build_nav(&chapter.children, current, path_to_root, section_labels));
        }

        html.push_str("</li>\n");
    }

    html.push_str("</ul>\n");
    html
}

/// Render a short piece of markdown, like a title, without a wrapping paragraph.
fn inline_markdown(md: &str) -> String {
    let mut options = comrak::Options::default();
    options.parse.smart = true;
    let html = comrak::markdown_to_html(md, &options);
    let html = html.trim();
    html.strip_prefix("<p>")
        .and_then(|h| h.strip_suffix("</p>"))
        .unwrap_or(html)
        .to_string()
}

/// A title as plain text, for the `<title>` element.
fn strip_inline_markdown(md: &str) -> String {
    md.replace('`', "")
}

/// What rendering one page needs to know about where it is.
struct PageContext<'a> {
    config: &'a Config,
    /// The page's path within the book, with `/` separators.
    page_url: &'a str,
    /// How many directories below the book root the page is.
    depth: usize,
    /// The book's rule identifiers and the page's `../` path back to the book
    /// root, set only for books using the Rust Reference's syntax. It lets a
    /// bare `[expr.array]` anywhere in the book resolve to the rule of that
    /// name.
    spec: Option<(&'a Spec, &'a str)>,
    library: Option<&'a LibraryBook>,
}

/// Render one chapter's markdown.
fn markdown_to_html(markdown: &str, ctx: &PageContext<'_>) -> String {
    let highlighter = Highlighter::new();

    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.alerts = true;
    // Give headings ids so links can target sections. The empty prefix keeps
    // the GitHub slug without GitHub's `user-content-` namespacing, which
    // exists to avoid colliding with GitHub's own page chrome and means
    // nothing here.
    options.extension.header_id_prefix = Some(String::new());
    options.parse.smart = ctx.config.smart_punctuation;
    // Book sources are trusted and hand-write HTML for things markdown can't
    // express. Without this comrak replaces each block with a comment saying
    // the raw HTML was omitted. mdbook passes it through too.
    options.render.r#unsafe = true;

    if let Some((spec, path_to_root)) = ctx.spec {
        options.parse.broken_link_callback = Some(Arc::new(
            move |link: comrak::options::BrokenLinkReference<'_>| {
                Some(comrak::ResolvedReference {
                    url: spec.rule_link(path_to_root, link.original)?,
                    title: link.original.to_string(),
                })
            },
        ));
    }

    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, markdown, &options);
    rewrite_links(root, ctx);
    apply_heading_attributes(&arena, root);
    render_code_blocks(root, ctx.config, &highlighter);

    let mut html = String::new();
    comrak::format_html(root, &options, &mut html)
        .expect("writing to a String never fails");
    html
}

/// Point links at the pages we actually render.
///
/// That includes links written as raw HTML, which some books generate.
fn rewrite_links(root: comrak::Node<'_>, ctx: &PageContext<'_>) {
    use comrak::nodes::NodeValue;

    for node in root.descendants() {
        let mut ast = node.data.borrow_mut();
        match &mut ast.value {
            NodeValue::Link(link) | NodeValue::Image(link) => {
                if let Some(rewritten) = rewrite_link(&link.url, ctx) {
                    link.url = rewritten;
                }
            }
            NodeValue::HtmlBlock(block) => block.literal = rewrite_html_links(&block.literal, ctx),
            NodeValue::HtmlInline(html) => *html = rewrite_html_links(html, ctx),
            _ => {}
        }
    }
}

static HTML_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\b(href|src)="([^"]*)""#).unwrap());

fn rewrite_html_links(html: &str, ctx: &PageContext<'_>) -> String {
    HTML_LINK_RE
        .replace_all(html, |caps: &rmx::regex::Captures<'_>| {
            match rewrite_link(&caps[2], ctx) {
                Some(url) => format!("{}=\"{}\"", &caps[1], html_escape(&url)),
                None => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Rewrite one link, or return `None` to leave it alone.
///
/// Books link between their chapters by `.md` path, which become `.html`.
/// Books in the library also link to each other by relative paths that only
/// work where they are published upstream, like `../std/` from the Cargo
/// book on doc.rust-lang.org. Those are resolved against the book's upstream
/// URL, then pointed at our copy of the target book if we have one and at
/// the upstream site if we don't.
fn rewrite_link(url: &str, ctx: &PageContext<'_>) -> Option<String> {
    let local = md_link_to_html(url);
    let url = local.as_deref().unwrap_or(url);

    let Some(library) = ctx.library else {
        return local;
    };
    if url.starts_with('#') || url.starts_with("//") || has_scheme(url) {
        return local;
    }
    if !escapes_book(url, ctx.depth) {
        return local;
    }

    let base = rmx::url::Url::parse(&library.upstream_url).ok()?.join(ctx.page_url).ok()?;
    let absolute = base.join(url).ok()?;
    let absolute = absolute.as_str();

    let sibling = library
        .books
        .iter()
        .filter(|(upstream, _)| absolute.starts_with(upstream.as_str()))
        .max_by_key(|(upstream, _)| upstream.len());
    Some(match sibling {
        Some((upstream, slug)) => {
            let rest = &absolute[upstream.len()..];
            format!("{}../{slug}/{rest}", "../".repeat(ctx.depth))
        }
        None => absolute.to_string(),
    })
}

/// Whether a link starts with a scheme, like `https:` or `mailto:`.
fn has_scheme(url: &str) -> bool {
    static SCHEME_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*:").unwrap());
    SCHEME_RE.is_match(url)
}

/// Whether a relative link climbs out of the book from a page `depth`
/// directories below the book root.
fn escapes_book(url: &str, depth: usize) -> bool {
    let path = url.split(['#', '?']).next().expect("split always yields one part");
    let mut level = depth as isize;
    for part in path.split('/') {
        match part {
            ".." => level -= 1,
            "" | "." => {}
            _ => level += 1,
        }
        if level < 0 {
            return true;
        }
    }
    false
}

/// Rewrite one relative link to a markdown file, preserving any fragment.
///
/// Returns `None` for links that don't name a markdown file within the book,
/// which are left alone.
fn md_link_to_html(url: &str) -> Option<String> {
    // Skip same-page fragments, protocol-relative URLs, and anything with a
    // scheme (`https:`, `mailto:`).
    if url.starts_with('#') || url.starts_with("//") || has_scheme(url) {
        return None;
    }

    let (path, fragment) = match url.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (url, None),
    };
    // Some books carry mdbook's `?highlight=` query, which we have no use for.
    let path = path.split('?').next().expect("split always yields one part");
    path.strip_suffix(".md")?;
    let html = url_path(&html_path(Path::new(path)));
    // `url_path` drops a leading `./`, which is harmless, but keep links
    // looking like the source wrote them.
    let html = if path.starts_with("./") { format!("./{html}") } else { html };

    Some(match fragment {
        Some(fragment) => format!("{html}#{fragment}"),
        None => html,
    })
}

/// A trailing `{#id .class}` on a heading, as mdbook supports.
static HEADING_ATTRS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s*\{\s*((?:[#.][\w-]+|[\w-]+=\S+)(?:\s+(?:[#.][\w-]+|[\w-]+=\S+))*)\s*\}\s*$")
        .unwrap()
});

/// Turn mdbook's heading attributes into an anchor, rather than leaving
/// them in the heading text.
///
/// The heading keeps its generated id as well, so links to either work.
fn apply_heading_attributes<'a>(arena: &'a comrak::Arena<'a>, root: comrak::Node<'a>) {
    use comrak::nodes::NodeValue;

    for node in root.descendants() {
        if !matches!(node.data.borrow().value, NodeValue::Heading(_)) {
            continue;
        }
        let Some(last) = node.last_child() else {
            continue;
        };
        let id = {
            let mut ast = last.data.borrow_mut();
            let NodeValue::Text(text) = &mut ast.value else {
                continue;
            };
            let Some(caps) = HEADING_ATTRS_RE.captures(text) else {
                continue;
            };
            let id = caps[1]
                .split_whitespace()
                .find_map(|attr| attr.strip_prefix('#'))
                .map(str::to_string);
            let start = caps.get(0).expect("group 0 always matches").start();
            *text = text[..start].to_string().into();
            id
        };
        if let Some(id) = id {
            let anchor = format!("<span id=\"{}\"></span>", html_escape(&id));
            node.prepend(arena.alloc(NodeValue::HtmlInline(anchor).into()));
        }
    }
}

/// Fence attributes that mark a code block as Rust when they come first.
const RUST_ATTRS: &[&str] = &[
    "ignore", "should_panic", "no_run", "compile_fail", "edition2015", "edition2018",
    "edition2021", "edition2024", "editable", "noplayground", "mdbook-runnable",
];

/// Fence attributes The Rust Programming Language marks with Ferris, and
/// what it says about each.
const CODE_NOTES: &[(&str, &str)] = &[
    ("does_not_compile", "This code does not compile!"),
    ("panics", "This code panics!"),
    ("not_desired_behavior", "This code does not produce the desired behavior."),
];

/// Render code blocks ourselves.
///
/// Compared with comrak's own rendering, this highlights them, hides the
/// lines mdbook hides, keeps the fence's attributes as classes, and labels
/// code that is wrong on purpose. Like mdbook, a block without a language,
/// including an indented one, is plain text.
fn render_code_blocks(root: comrak::Node<'_>, config: &Config, highlighter: &Highlighter) {
    use comrak::nodes::NodeValue;

    for node in root.descendants() {
        let mut ast = node.data.borrow_mut();
        let NodeValue::CodeBlock(block) = &ast.value else {
            continue;
        };
        let mut words = block
            .info
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|w| !w.is_empty());
        let first = words.next().unwrap_or("");
        let (lang, attrs): (&str, Vec<&str>) = if first.is_empty() {
            ("text", Vec::new())
        } else if RUST_ATTRS.contains(&first) {
            ("rust", std::iter::once(first).chain(words).collect())
        } else {
            (first, words.collect())
        };

        if lang == "mermaid" {
            ast.value = NodeValue::Raw(format!(
                "<pre class=\"mermaid\">{}</pre>\n",
                html_escape(&block.literal)
            ));
            continue;
        }

        let code = if lang == "rust" {
            hide_rust_lines(&block.literal)
        } else if let Some(prefix) = config.hidelines.get(lang) {
            hide_prefixed_lines(&block.literal, prefix)
        } else {
            block.literal.clone()
        };

        let highlighted = if lang == "text" {
            html_escape(&code)
        } else {
            highlighter.highlight(&code, lang)
        };

        let mut html = String::new();
        for (attr, note) in CODE_NOTES {
            if attrs.contains(attr) {
                html.push_str(&format!("<p class=\"code-note {attr}\">{note}</p>\n"));
            }
        }
        let classes: String = attrs
            .iter()
            .filter(|a| a.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .map(|a| format!(" {a}"))
            .collect();
        html.push_str(&format!(
            "<pre class=\"highlight{classes}\"><code class=\"language-{}\">{highlighted}</code></pre>\n",
            html_escape(lang)
        ));
        ast.value = NodeValue::Raw(html);
    }
}

/// Drop the lines of a Rust example that mdbook hides.
///
/// A line whose first non-blank character is `#` followed by a space or
/// nothing is hidden. `##` stands for a literal `#`, and anything else, like
/// `#[derive]`, is ordinary code.
fn hide_rust_lines(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    for line in code.split_inclusive('\n') {
        let content = line.trim_end_matches('\n');
        let indent = &content[..content.len() - content.trim_start().len()];
        let rest = content.trim_start();
        if let Some(after) = rest.strip_prefix('#') {
            if after.is_empty() || after.starts_with(' ') {
                continue;
            }
            if after.starts_with('#') {
                out.push_str(indent);
                out.push_str(after);
                if line.ends_with('\n') {
                    out.push('\n');
                }
                continue;
            }
        }
        out.push_str(line);
    }
    out
}

fn hide_prefixed_lines(code: &str, prefix: &str) -> String {
    code.split_inclusive('\n')
        .filter(|line| !line.trim_start().starts_with(prefix))
        .collect()
}

/// Make the book's root page.
///
/// mdbook makes the first chapter the root page. We redirect to it instead,
/// unless a root `README.md` already rendered as `index.html`.
fn generate_index(book: &Book, output: &Path) -> AnyResult<()> {
    let first_chapter = flatten_chapters(&book.chapters)
        .into_iter()
        .find_map(|c| c.path.as_ref());

    let Some(first) = first_chapter else {
        return Err(anyhow!("No chapters found to redirect index.html to"));
    };
    let target = url_path(&html_path(first));
    if target == "index.html" {
        return Ok(());
    }

    let index_html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0; url={target}">
    <title>{title}</title>
    <link rel="stylesheet" href="rustmax-themes.css">
    <link rel="stylesheet" href="rustmax-syntax.css">
    <link rel="stylesheet" href="rmxbook.css">
</head>
<body>
</body>
</html>"#,
        title = html_escape(&book.title),
    );

    fs::write(output.join("index.html"), index_html)?;

    Ok(())
}

/// Leave a page at each old path a book redirects from, sending readers on
/// to where the content moved, as mdbook does.
///
/// Redirects of a fragment within a page, which mdbook handles with script,
/// are skipped, as are any that would replace a real page.
fn generate_redirects(
    config: &Config,
    output: &Path,
    library: Option<&LibraryBook>,
) -> AnyResult<()> {
    for (from, to) in &config.redirects {
        if from.contains('#') {
            continue;
        }
        let from = from.trim_start_matches('/').trim_start_matches("./");
        let stub = output.join(from);
        if stub.exists() {
            continue;
        }

        let depth = from.matches('/').count();
        // A leading `/` is relative to the book's root.
        let to = match to.strip_prefix('/') {
            Some(rooted) => format!("{}{rooted}", "../".repeat(depth)),
            None => to.clone(),
        };
        let ctx = PageContext { config, page_url: from, depth, spec: None, library };
        let target = rewrite_link(&to, &ctx).unwrap_or(to);
        let target = html_escape(&target);

        if let Some(parent) = stub.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &stub,
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0; url={target}">
    <link rel="canonical" href="{target}">
    <title>Redirecting...</title>
</head>
<body>
    <p>This page has moved to <a href="{target}">{target}</a>.</p>
</body>
</html>"#
            ),
        )?;
    }
    Ok(())
}

const MERMAID_SCRIPT: &str = r#"
    <script type="module">
      import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
      mermaid.initialize({ startOnLoad: true, theme: matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "default" });
    </script>"#;

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
  document.addEventListener('DOMContentLoaded', function() {
    // Bring the current chapter into view in a long sidebar.
    var sidebar = document.querySelector('.sidebar');
    var current = sidebar && sidebar.querySelector('.current > a');
    if (current) {
      sidebar.scrollTop = current.offsetTop - sidebar.clientHeight / 3;
    }
  });
  // Arrow keys turn the page, as in mdbook.
  document.addEventListener('keydown', function(e) {
    if (e.altKey || e.ctrlKey || e.metaKey || e.shiftKey) return;
    var t = e.target;
    if (t && (t.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(t.tagName))) return;
    var rel = e.key === 'ArrowLeft' ? 'prev' : e.key === 'ArrowRight' ? 'next' : null;
    var link = rel && document.querySelector('.nav-links a.' + rel);
    if (link) window.location.href = link.href;
  });
})();"#
}

fn generate_css(config: &Config) -> String {
    let mut css = base_css();
    if config.spec {
        css.push_str(spec::CSS);
        css.push_str(spec::GRAMMAR_CSS);
    }
    if config.listings {
        css.push_str(listing::CSS);
    }
    css
}

fn base_css() -> String {
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
    font-feature-settings: "onum" 1, "kern" 1;
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
.sidebar .section-number { color: var(--rmx-color-fg); opacity: 0.6; }
.sidebar .draft { opacity: 0.6; }
.sidebar .part-title {
    font: var(--rmx-font-em);
    margin-top: 1.2rem;
    white-space: normal;
    color: var(--rmx-color-accents);
}
.sidebar .separator {
    height: 0;
    margin: 0.8rem 0;
    border-top: 1px dashed var(--rmx-color-border);
}

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
    color: var(--rmx-color-accents);
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    line-height: 1.3;
}

h1 { font-size: 2rem; letter-spacing: -0.02em; }
h2 { font-size: 1.5rem; letter-spacing: -0.01em; }
h3 { font-size: 1.25rem; }

code {
    font: var(--rmx-font-code);
    font-feature-settings: "lnum" 1;
    font-size: 0.9em;
    background: var(--rmx-color-bg-alt);
    padding: 0.15em 0.3em;
    border-radius: 3px;
}

pre {
    background: var(--rmx-color-bg-alt);
    border-left: 3px solid var(--rmx-color-accents);
    border-radius: 0 3px 3px 0;
    padding: 1rem;
    overflow-x: auto;
}

pre code { background: none; padding: 0; }

img, svg, video { max-width: 100%; height: auto; }

blockquote {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 3px solid var(--rmx-color-accents);
    border-radius: 0 3px 3px 0;
    background: var(--rmx-color-bg-alt);
}

/* Tables size to their content and scroll sideways when too wide,
   rather than pushing the page wider than the screen. */
table { border-collapse: collapse; margin: 1rem 0; display: block; overflow-x: auto; max-width: 100%; }
th, td { border: none; border-bottom: 1px solid var(--rmx-color-border); padding: 0.5rem; text-align: left; }
th { background: var(--rmx-color-bg-alt); border-bottom-width: 2px; }

dt { font-weight: bold; }
dd { margin-left: 1.5rem; }
dd > p:first-child { margin-top: 0; }

/* GitHub-style `> [!NOTE]` alerts. */
.markdown-alert {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 3px solid var(--rmx-color-links);
    border-radius: 0 3px 3px 0;
    background: var(--rmx-color-bg-alt);
}
.markdown-alert-title { font: var(--rmx-font-em); margin: 0; color: var(--rmx-color-links); }
.markdown-alert-warning, .markdown-alert-caution { border-left-color: var(--rmx-color-accents); }
.markdown-alert-warning .markdown-alert-title,
.markdown-alert-caution .markdown-alert-title { color: var(--rmx-color-accents); }

/* Code that is wrong on purpose, from fence attributes like `does_not_compile`. */
.code-note {
    margin: 1rem 0 -0.75rem;
    padding: 0.25rem 1rem;
    font-size: 0.85em;
    font-style: italic;
    color: var(--rmx-color-accents);
    border-left: 3px dashed var(--rmx-color-accents);
}
.code-note + pre { border-left-style: dashed; }

pre.mermaid { border-left: none; background: none; text-align: center; }

/* Per-book markup that upstream styles in its own theme. */
a.option-anchor, a.option-anchor > * { color: inherit; }
span.caption { font-size: 0.85em; font-weight: bold; }
.warning {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border: 2px solid var(--rmx-color-accents);
    border-radius: 3px;
}

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

.generated-elsewhere {
    padding: 0.5rem 1rem;
    border: 1px dashed var(--rmx-color-border);
    font-style: italic;
}

.provenance {
    margin-top: 2rem;
    font-size: 0.85em;
    opacity: 0.8;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        Config {
            title: "Test".to_string(),
            src_dir: PathBuf::new(),
            spec: false,
            listings: false,
            smart_punctuation: false,
            section_labels: true,
            hidelines: BTreeMap::new(),
            redirects: Vec::new(),
        }
    }

    fn render_at(md: &str, page_url: &str, library: Option<&LibraryBook>) -> String {
        let config = config();
        let ctx = PageContext {
            config: &config,
            page_url,
            depth: page_url.matches('/').count(),
            spec: None,
            library,
        };
        markdown_to_html(md, &ctx)
    }

    fn render(md: &str) -> String {
        render_at(md, "index.html", None)
    }

    fn library() -> LibraryBook {
        LibraryBook {
            name: "The Cargo Book".to_string(),
            upstream_url: "https://doc.rust-lang.org/cargo/".to_string(),
            source_url: "https://github.com/rust-lang/cargo/tree/abc".to_string(),
            commit: "abc".to_string(),
            books: vec![
                ("https://doc.rust-lang.org/cargo/".to_string(), "cargo-book".to_string()),
                ("https://doc.rust-lang.org/book/".to_string(), "trpl".to_string()),
                ("https://doc.rust-lang.org/reference/".to_string(), "reference".to_string()),
            ],
        }
    }

    #[test]
    fn test_md_links_become_html_links() {
        assert_eq!(md_link_to_html("types/array.md").unwrap(), "types/array.html");
        assert_eq!(md_link_to_html("../types/array.md").unwrap(), "../types/array.html");
        assert_eq!(md_link_to_html("./array.md").unwrap(), "./array.html");
        assert_eq!(
            md_link_to_html("../const_eval.md#constant-expressions").unwrap(),
            "../const_eval.html#constant-expressions"
        );
        assert_eq!(md_link_to_html("a b.md").unwrap(), "a b.html");
        assert_eq!(
            md_link_to_html("build.md?highlight=rustup#creating").unwrap(),
            "build.html#creating"
        );
    }

    #[test]
    fn test_readme_links_become_index_links() {
        assert_eq!(md_link_to_html("cli/README.md").unwrap(), "cli/index.html");
        assert_eq!(md_link_to_html("../README.md#x").unwrap(), "../index.html#x");
        assert_eq!(html_path(Path::new("format/README.md")), Path::new("format/index.html"));
    }

    #[test]
    fn test_non_md_links_are_left_alone() {
        assert_eq!(md_link_to_html("#anchor"), None);
        assert_eq!(md_link_to_html("index.html"), None);
        assert_eq!(md_link_to_html("img/diagram.svg"), None);
        assert_eq!(md_link_to_html(""), None);
        // A markdown file on another site is somebody else's source, not a
        // page we render.
        assert_eq!(md_link_to_html("https://example.com/README.md"), None);
        assert_eq!(md_link_to_html("//example.com/README.md"), None);
        assert_eq!(md_link_to_html("mailto:nobody@example.com"), None);
    }

    #[test]
    fn test_markdown_links_are_rewritten_in_output() {
        let html = render("See [arrays](types/array.md#len) and [the web](https://example.com/x.md).");
        assert!(html.contains(r#"href="types/array.html#len""#), "{html}");
        assert!(html.contains(r#"href="https://example.com/x.md""#), "{html}");
    }

    #[test]
    fn test_image_links_are_rewritten() {
        let html = render("![alt](sub/page.md)");
        assert!(html.contains(r#"src="sub/page.html""#), "{html}");
    }

    #[test]
    fn test_links_to_other_library_books_stay_in_the_library() {
        let lib = library();
        let html = render_at("[ch1](../../book/ch01.html) and [r](../../reference/x.md#y)", "guide/index.html", Some(&lib));
        assert!(html.contains(r#"href="../../trpl/ch01.html""#), "{html}");
        assert!(html.contains(r#"href="../../reference/x.html#y""#), "{html}");
    }

    #[test]
    fn test_links_to_books_outside_the_library_go_upstream() {
        let lib = library();
        let html = render_at("[Vec](../../std/vec/struct.Vec.html)", "reference/x.html", Some(&lib));
        assert!(html.contains(r#"href="https://doc.rust-lang.org/std/vec/struct.Vec.html""#), "{html}");
    }

    #[test]
    fn test_cross_book_links_keep_their_query() {
        let lib = library();
        let html = render_at("[s](../../std/vec/struct.Vec.html?search=vec::new&filter-crate=std)", "a/b.html", Some(&lib));
        assert!(html.contains(r#"href="https://doc.rust-lang.org/std/vec/struct.Vec.html?search=vec::new&amp;filter-crate=std""#), "{html}");
    }

    #[test]
    fn test_links_within_a_library_book_are_untouched() {
        let lib = library();
        let html = render_at("[up](../index.html) [down](sub/a.html)", "guide/x.html", Some(&lib));
        assert!(html.contains(r#"href="../index.html""#), "{html}");
        assert!(html.contains(r#"href="sub/a.html""#), "{html}");
    }

    #[test]
    fn test_raw_html_links_are_rewritten() {
        let lib = library();
        let html = render_at("<a href=\"../../book/ch01.html\">ch1</a> <a href=\"x.md\">x</a>\n", "guide/index.html", Some(&lib));
        assert!(html.contains(r#"href="../../trpl/ch01.html""#), "{html}");
        assert!(html.contains(r#"href="x.html""#), "{html}");
    }

    #[test]
    fn test_escapes_book() {
        assert!(escapes_book("../std/x.html", 0));
        assert!(!escapes_book("../x.html", 1));
        assert!(escapes_book("../../x.html", 1));
        assert!(!escapes_book("a/../../x.html", 1));
        assert!(!escapes_book("./x.html", 0));
    }

    #[test]
    fn test_raw_html_passes_through() {
        let html = render("Rust<sup>1</sup>\n\n<div class=\"note\">\n\nhi\n\n</div>\n");
        assert!(html.contains("<sup>1</sup>"), "{html}");
        assert!(html.contains(r#"<div class="note">"#), "{html}");
        assert!(!html.contains("raw HTML omitted"), "{html}");
    }

    #[test]
    fn test_md_links_in_code_are_left_alone() {
        let html = render("```text\n<a href=\"foo.md\">x</a>\n```\n");
        assert!(html.contains("foo.md"), "{html}");
    }

    #[test]
    fn test_headings_get_ids() {
        let html = render("## Graveyard\n");
        assert!(html.contains(r#"<h2 id="graveyard">"#), "{html}");
    }

    /// GitHub namespaces ids to avoid colliding with its own page chrome.
    /// We render the whole page, so the prefix is just noise, and the
    /// sitemap links without it.
    #[test]
    fn test_heading_ids_are_not_github_prefixed() {
        let html = render("## Graveyard\n");
        assert!(!html.contains("user-content-"), "{html}");
    }

    /// Slugs follow the GitHub rules the sitemap was written against:
    /// keep letters, numbers, `_` and `-`, drop everything else,
    /// spaces become hyphens.
    #[test]
    fn test_heading_slugs_match_the_github_rules() {
        for (heading, id) in [
            ("### 🌞 `cargo-edit`", "-cargo-edit"),
            ("## Must know URLS 🤯", "must-know-urls-"),
            ("## Assert `Send` / `Sync`", "assert-send--sync"),
            ("### Use `rustfmt::skip`", "use-rustfmtskip"),
            ("### … create a fast `HashMap`?", "-create-a-fast-hashmap"),
            ("## Logging - `log` and `env_logger`", "logging---log-and-env_logger"),
        ] {
            let html = render(&format!("{heading}\n"));
            assert!(html.contains(&format!("id=\"{id}\"")), "{heading} -> {html}");
        }
    }

    #[test]
    fn test_repeated_headings_get_distinct_ids() {
        let html = render("## Topics\n\n## Topics\n");
        assert!(html.contains(r#"id="topics""#), "{html}");
        assert!(html.contains(r#"id="topics-1""#), "{html}");
    }

    #[test]
    fn test_heading_attributes_become_an_anchor() {
        let html = render("#### Minor: adding a feature {#cargo-feature-add}\n");
        assert!(html.contains(r#"<span id="cargo-feature-add"></span>"#), "{html}");
        assert!(!html.contains("{#"), "{html}");
        assert!(html.contains(r#"id="minor-adding-a-feature""#), "{html}");
    }

    #[test]
    fn test_heading_braces_that_are_not_attributes_stay() {
        let html = render("## The `{}` placeholder\n\n## Set {a, b}\n");
        assert!(html.contains("{a, b}"), "{html}");
        assert!(!html.contains("<span id="), "{html}");
    }

    #[test]
    fn test_alerts() {
        let html = render("> [!NOTE]\n> Something to know.\n");
        assert!(html.contains("markdown-alert-note"), "{html}");
        assert!(!html.contains("[!NOTE]"), "{html}");
    }

    #[test]
    fn test_hidden_rust_lines_are_dropped() {
        let code = "# fn main() {\n#\nlet x = 1;\n#[derive(Debug)]\n#![allow(x)]\n## not hidden\n    # indented\n# }\n";
        assert_eq!(
            hide_rust_lines(code),
            "let x = 1;\n#[derive(Debug)]\n#![allow(x)]\n# not hidden\n"
        );
        let html = render(&format!("```rust\n{code}```\n"));
        assert!(!html.contains("main"), "{html}");
    }

    #[test]
    fn test_hidden_lines_follow_the_fence_attributes() {
        let html = render("```rust,ignore\n# hidden\nshown\n```\n\n```console\n# root prompt\n```\n");
        assert!(!html.contains("hidden"), "{html}");
        assert!(html.contains("root prompt"), "{html}");
        assert!(html.contains(r#"<pre class="highlight ignore">"#), "{html}");
    }

    #[test]
    fn test_unlabeled_blocks_are_plain_text() {
        let html = render("```\n# a shell comment\n```\n\n    # indented\n");
        assert!(html.contains("<code class=\"language-text\"># a shell comment"), "{html}");
        assert!(html.contains("<code class=\"language-text\"># indented"), "{html}");
    }

    #[test]
    fn test_custom_hidelines() {
        let mut config = config();
        config.hidelines.insert("python".to_string(), "~".to_string());
        let ctx = PageContext { config: &config, page_url: "x.html", depth: 0, spec: None, library: None };
        let html = markdown_to_html("```python\n~hidden\nshown\n```\n", &ctx);
        assert!(!html.contains("hidden"), "{html}");
        assert!(html.contains("shown"), "{html}");
    }

    #[test]
    fn test_deliberately_broken_code_is_labeled() {
        let html = render("```rust,ignore,does_not_compile\nfn x() {\n```\n");
        assert!(html.contains("This code does not compile!"), "{html}");
        assert!(html.contains(r#"class="highlight ignore does_not_compile""#), "{html}");
    }

    #[test]
    fn test_mermaid_blocks_are_left_for_mermaid() {
        let html = render("```mermaid\ngraph TD; A-->B;\n```\n");
        assert!(html.contains(r#"<pre class="mermaid">graph TD; A--&gt;B;"#), "{html}");
    }

    #[test]
    fn test_smart_punctuation() {
        let mut config = config();
        config.smart_punctuation = true;
        let ctx = PageContext { config: &config, page_url: "x.html", depth: 0, spec: None, library: None };
        let html = markdown_to_html("\"fat\" LTO, rustc's `\"x\"`", &ctx);
        assert!(html.contains("“fat”"), "{html}");
        assert!(html.contains("rustc’s"), "{html}");
        assert!(html.contains("&quot;x&quot;"), "{html}");
    }

    #[test]
    fn test_summary_entry_with_a_trailing_note() {
        let chapter =
            parse_chapter_link("- [Rust Concept Index](concept-index.md) (aka \"glossary\")")
                .unwrap();
        assert_eq!(chapter.title, "Rust Concept Index");
        assert_eq!(chapter.path, Some(PathBuf::from("concept-index.md")));
    }

    #[test]
    fn test_summary_entry_title_may_contain_parens() {
        let chapter = parse_chapter_link("- [Rust (2024) Edition](edition.md)").unwrap();
        assert_eq!(chapter.title, "Rust (2024) Edition");
        assert_eq!(chapter.path, Some(PathBuf::from("edition.md")));
    }

    fn outline(chapters: &[Chapter]) -> Vec<String> {
        fn walk(chapters: &[Chapter], depth: usize, out: &mut Vec<String>) {
            for c in chapters {
                let label = match c.kind {
                    ChapterKind::Page => format!("{}{}", c.number.as_deref().map(|n| format!("{n} ")).unwrap_or_default(), c.title),
                    ChapterKind::PartTitle => format!("# {}", c.title),
                    ChapterKind::Separator => "---".to_string(),
                };
                out.push(format!("{}{label}", "  ".repeat(depth)));
                walk(&c.children, depth + 1, out);
            }
        }
        let mut out = Vec::new();
        walk(chapters, 0, &mut out);
        out
    }

    #[test]
    fn test_summary_nesting_with_four_space_indents() {
        let summary = "# Summary\n\n[Intro](intro.md)\n\n- [A](a.md)\n    - [A1](a1.md)\n    - [A2](a2.md)\n        - [A2x](a2x.md)\n    - [A3](a3.md)\n- [B](b.md)\n";
        assert_eq!(
            outline(&parse_summary(summary)),
            ["Intro", "1. A", "  1.1. A1", "  1.2. A2", "    1.2.1. A2x", "  1.3. A3", "2. B"]
        );
    }

    #[test]
    fn test_summary_nesting_with_mixed_indents() {
        let summary = "- [A](a.md)\n  - [A1](a1.md)\n      - [A1x](a1x.md)\n  - [A2](a2.md)\n";
        assert_eq!(outline(&parse_summary(summary)), ["1. A", "  1.1. A1", "    1.1.1. A1x", "  1.2. A2"]);
    }

    #[test]
    fn test_summary_parts_separators_and_drafts() {
        let summary = "# Summary\n\n[Intro](intro.md)\n\n# Part one\n\n- [A](a.md)\n- [Draft]()\n\n# Part two\n\n- [B](b.md)\n\n---\n\n[Appendix](appendix.md)\n";
        assert_eq!(
            outline(&parse_summary(summary)),
            ["Intro", "# Part one", "1. A", "2. Draft", "# Part two", "3. B", "---", "Appendix"]
        );
    }

    #[test]
    fn test_redirect_stubs() {
        let dir = rmx::tempfile::tempdir().unwrap();
        fs::write(dir.path().join("real.html"), "real").unwrap();
        let mut config = config();
        config.redirects = vec![
            ("ch17-00-oop.html".to_string(), "ch18-00-oop.html".to_string()),
            ("/old/deep.html".to_string(), "/new/page.html".to_string()),
            ("/page.html#old".to_string(), "#new".to_string()),
            ("real.html".to_string(), "elsewhere.html".to_string()),
        ];
        generate_redirects(&config, dir.path(), None).unwrap();

        let read = |p: &str| fs::read_to_string(dir.path().join(p)).unwrap();
        assert!(read("ch17-00-oop.html").contains(r#"url=ch18-00-oop.html""#));
        assert!(read("old/deep.html").contains(r#"url=../new/page.html""#));
        assert_eq!(read("real.html"), "real");
        assert!(!dir.path().join("page.html").exists());
    }

    /// `maintainers.md` writes its own anchors, which must keep working
    /// alongside the generated ones.
    #[test]
    fn test_hand_written_anchors_survive() {
        let html = render("## <a id=\"dtolnay\"></a> David Tolnay\n");
        assert!(html.contains(r#"<a id="dtolnay"></a>"#), "{html}");
    }
}
