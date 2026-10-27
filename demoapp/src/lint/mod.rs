//! Content lints reported by `folio check`.
//!
//! Folio is aimed at technical writing, where most of what goes wrong in a
//! document is not the prose but the code around it. These lints read the
//! parsed markdown rather than the rendered HTML, so they can point at the
//! source line the author wrote.

use rmx::comrak::nodes::NodeValue;
use rmx::comrak::{Arena, Options, parse_document};
use rmx::jiff::civil::Date;
use rmx::rayon::prelude::*;
use rmx::regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use rmx::prelude::*;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::collection::{Collection, Config, Document};
use crate::site::{SiteIndex, Target};

/// How seriously to take a lint.
#[rmx::derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Level {
    /// Worth reporting, but does not fail `check`.
    #[display("warning")]
    Warning,
    /// Fails `check`.
    #[display("error")]
    Error,
}

/// A problem found in a document.
#[derive(Debug, Clone)]
pub struct Lint {
    pub path: PathBuf,
    /// 1-based line in the source document.
    pub line: usize,
    pub level: Level,
    pub message: String,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}: {}",
            self.path.display(),
            self.line,
            self.level,
            self.message,
        )
    }
}

/// A fenced code block, as written in the source document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    /// 1-based line of the opening fence.
    pub line: usize,
    /// The full info string, e.g. `rust,ignore`.
    pub info: String,
    pub code: String,
}

impl CodeBlock {
    /// The language token, the part of the info string before any comma.
    pub fn language(&self) -> &str {
        self.info.split(',').next().unwrap_or("").trim()
    }

    /// The comma-separated attributes following the language.
    pub fn attributes(&self) -> impl Iterator<Item = &str> {
        self.info.split(',').skip(1).map(str::trim)
    }

    /// Whether this block holds Rust.
    pub fn is_rust(&self) -> bool {
        matches!(self.language(), "rust" | "rs")
    }

    /// Whether the author has asked for this block not to be checked.
    ///
    /// These are the rustdoc attributes that mean the text is deliberately not
    /// valid Rust, or is not meant to stand alone.
    pub fn is_exempt(&self) -> bool {
        self.attributes()
            .any(|attr| matches!(attr, "ignore" | "compile_fail" | "text" | "no_check"))
    }

    /// The code with rustdoc's hidden lines removed.
    ///
    /// A line whose first non-space character is `#` followed by a space, or a
    /// lone `#`, is hidden from readers by rustdoc but is part of the program.
    /// `##` is an escape for a literal leading `#`.
    pub fn visible_and_hidden_source(&self) -> String {
        self.code
            .lines()
            .map(|line| {
                let trimmed = line.trim_start();
                if let Some(rest) = trimmed.strip_prefix("##") {
                    format!("#{rest}")
                } else if trimmed == "#" {
                    String::new()
                } else if let Some(rest) = trimmed.strip_prefix("# ") {
                    rest.to_owned()
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Extract the fenced code blocks of a markdown document, in source order.
pub fn code_blocks(markdown: &str) -> Vec<CodeBlock> {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &Options::default());

    root.descendants()
        .filter_map(|node| {
            let data = node.data.borrow();
            let NodeValue::CodeBlock(block) = &data.value else {
                return None;
            };
            // Indented blocks have no info string and no language to check.
            block.fenced.then(|| CodeBlock {
                line: data.sourcepos.start.line,
                info: block.info.clone(),
                code: block.literal.clone(),
            })
        })
        .collect()
}

/// Check that every Rust code block in a document parses.
///
/// A block is tried first as a whole file, then, if that fails, as the body of
/// a `fn main`, which is how rustdoc treats a snippet with no `fn main` of its
/// own. Only a block that fails both ways is reported.
pub fn check_rust_syntax(path: &Path, markdown: &str) -> Vec<Lint> {
    code_blocks(markdown)
        .iter()
        .filter(|block| block.is_rust() && !block.is_exempt())
        .filter_map(|block| {
            let source = block.visible_and_hidden_source();

            let as_file = match rmx::syn::parse_file(&source) {
                Ok(_) => return None,
                Err(e) => e,
            };

            if rmx::syn::parse_file(&format!("fn main() {{\n{source}\n}}")).is_ok() {
                return None;
            }

            Some(Lint {
                path: path.to_path_buf(),
                line: block.line,
                level: Level::Error,
                message: format!("Rust code block does not parse: {as_file}"),
            })
        })
        .collect()
}

/// Check that a code block that looks like it has a language actually names one
/// Folio can highlight.
///
/// An unknown language is not an error -- it renders as plain text -- but it is
/// usually a typo, and silently unhighlighted code is easy to miss.
pub fn check_code_block_languages(path: &Path, markdown: &str) -> Vec<Lint> {
    code_blocks(markdown)
        .iter()
        .filter(|block| !block.language().is_empty())
        .filter(|block| crate::build::languages::by_name(block.language()).is_none())
        .map(|block| Lint {
            path: path.to_path_buf(),
            line: block.line,
            level: Level::Warning,
            message: format!(
                "code block language `{}` is not highlighted; it renders as plain text \
                 (highlighted: {})",
                block.language(),
                crate::build::languages::supported().join(", "),
            ),
        })
        .collect()
}

/// Check that every shortcode in a document is one Folio renders.
///
/// An unknown shortcode is left in the page as written,
/// which is rarely what the author meant.
pub fn check_shortcodes(path: &Path, markdown: &str) -> Vec<Lint> {
    crate::shortcode::find_shortcodes(markdown)
        .into_iter()
        .filter(|found| !found.shortcode.is_builtin())
        .map(|found| Lint {
            path: path.to_path_buf(),
            line: line_of(markdown, found.span.start),
            level: Level::Warning,
            message: format!(
                "unknown shortcode `{}` is left in the page as written",
                found.shortcode.name,
            ),
        })
        .collect()
}

/// A link or image reference as written in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// 1-based line in the document's content.
    pub line: usize,
    pub url: String,
    pub is_image: bool,
}

/// Every link and image in a markdown document, in source order,
/// including the images of `figure` shortcodes.
pub fn references(markdown: &str) -> Vec<Reference> {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &Options::default());

    let mut found: Vec<Reference> = root
        .descendants()
        .filter_map(|node| {
            let data = node.data.borrow();
            let (link, is_image) = match &data.value {
                NodeValue::Link(link) => (link, false),
                NodeValue::Image(link) => (link, true),
                _ => return None,
            };
            Some(Reference { line: data.sourcepos.start.line, url: link.url.clone(), is_image })
        })
        .collect();

    for shortcode in crate::shortcode::find_shortcodes(markdown) {
        if shortcode.shortcode.name == "figure"
            && let Some(src) = shortcode.shortcode.get("src").or_else(|| shortcode.shortcode.positional(0))
        {
            found.push(Reference {
                line: line_of(markdown, shortcode.span.start),
                url: src.to_string(),
                is_image: true,
            });
        }
    }

    found.sort_by_key(|reference| reference.line);
    found
}

/// The 1-based line holding byte `offset`.
fn line_of(text: &str, offset: usize) -> usize {
    rmx::memchr::memchr_iter(b'\n', &text.as_bytes()[..offset]).count() + 1
}

/// The `#fragment` of a link, decoded, if it has a non-empty one.
fn fragment(url: &str) -> Option<String> {
    let (_, fragment) = url.split_once('#')?;
    (!fragment.is_empty()).then(|| crate::site::percent_decode(fragment))
}

/// Check that every link and image in a document leads somewhere the build writes,
/// and that every `#fragment` names an id on the page it leads to.
pub fn check_references(doc: &Document, context: &LintContext) -> Vec<Lint> {
    let site = &context.site;
    let lint = |reference: &Reference, level, message| Lint {
        path: doc.source_path.clone(),
        line: reference.line,
        level,
        message,
    };

    references(&doc.content)
        .iter()
        .filter_map(|reference| {
            let url = &reference.url;
            let target = site.resolve(doc, url);

            // The page a fragment has to be found on, if it is a document's.
            let page = match &target {
                Target::SamePage => Some(doc.url_path()),
                Target::Document { url } => Some(url.clone()),
                _ => None,
            };
            if !reference.is_image
                && let Some(page) = page
                && let Some(fragment) = fragment(url)
                && !context.anchors.get(&page).is_some_and(|ids| ids.contains(&fragment))
            {
                return Some(lint(
                    reference,
                    Level::Error,
                    format!("link to `{url}`: {page} has no heading or anchor `{fragment}`"),
                ));
            }

            match (target, reference.is_image) {
                (Target::External | Target::SamePage | Target::Built, false) => None,
                (Target::Document { .. } | Target::Static(_), false) => None,
                (Target::External | Target::Built, true) => None,
                (Target::Static(path), true) => match rmx::image::image_dimensions(&path) {
                    // Formats the build cannot measure, SVG among them, are still fine to link.
                    Ok(_) | Err(rmx::image::ImageError::Unsupported(_)) => None,
                    Err(e) => Some(lint(reference, Level::Error, format!("image `{url}` cannot be read: {e}"))),
                },
                (Target::Draft { url: page }, false) => Some(lint(
                    reference,
                    Level::Warning,
                    format!("link to `{url}` leads to the draft {page}, which is only built with drafts"),
                )),
                (Target::Missing, false) => {
                    Some(lint(reference, Level::Error, format!("link to `{url}` leads nowhere in the built site")))
                }
                (_, true) => Some(lint(
                    reference,
                    Level::Error,
                    format!("image `{url}` is not a file in `static/` or anywhere else in the built site"),
                )),
            }
        })
        .collect()
}

/// Warn about a published document dated after `today`.
///
/// Folio publishes it anyway, so a date in the future is usually a typo
/// or a post meant to stay a draft.
pub fn check_date(doc: &Document, today: Date) -> Option<Lint> {
    let date = doc.frontmatter.date?;
    (date > today && !doc.frontmatter.draft).then(|| Lint {
        path: doc.source_path.clone(),
        // The frontmatter starts on the first line.
        line: 1,
        level: Level::Warning,
        message: format!("dated {date}, after today ({today}), but not a draft; it is published now"),
    })
}

/// What lints need to know beyond the document they are checking.
#[derive(Debug, Clone)]
pub struct LintContext {
    pub site: SiteIndex,
    pub today: Date,
    /// The ids on each document's page, by page URL: what a `#fragment` can name.
    pub anchors: HashMap<String, HashSet<String>>,
}

impl LintContext {
    pub fn new(collection: &Collection, config: &Config) -> crate::Result<Self> {
        static ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\bid="([^"]+)""#).expect("valid regex"));

        // Rendered the way the build renders them, so the ids are the ones
        // the pages will have: heading ids, footnotes, anchors in raw HTML.
        let anchors = collection
            .documents
            .par_iter()
            .map(|doc| {
                let html = crate::build::render_document(doc, config);
                let ids = ID.captures_iter(&html).map(|caps| caps[1].to_string()).collect();
                (doc.url_path(), ids)
            })
            .collect();

        Ok(Self {
            site: SiteIndex::new(collection, config)?,
            today: config.build_time().date(),
            anchors,
        })
    }
}

/// Run every lint against a document.
///
/// Lines are reported against the source file, frontmatter included, so they
/// match what the author sees in an editor.
pub fn check_document(doc: &Document, context: &LintContext) -> Vec<Lint> {
    let path = &doc.source_path;
    let mut lints = check_rust_syntax(path, &doc.content);
    lints.extend(check_code_block_languages(path, &doc.content));
    lints.extend(check_shortcodes(path, &doc.content));
    lints.extend(check_references(doc, context));

    for lint in &mut lints {
        lint.line = doc.source_line(lint.line);
    }

    lints.extend(check_date(doc, context.today));
    lints.sort_by_key(|lint| lint.line);
    lints
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A context in which `doc` is the whole collection.
    fn context_for(doc: &Document) -> LintContext {
        let collection = Collection { root: PathBuf::from("no-such-collection"), documents: vec![doc.clone()] };
        LintContext::new(&collection, &Config::default()).unwrap()
    }

    #[test]
    fn references_include_links_images_and_figures() {
        let md = "A [link](a.md) and ![img](/b.png).\n\n{{< figure src=\"/c.png\" >}}\n\n```\n[not](a link)\n```\n";
        let found = references(md);
        let urls: Vec<_> = found.iter().map(|r| (r.url.as_str(), r.is_image, r.line)).collect();
        assert_eq!(urls, [("a.md", false, 1), ("/b.png", true, 1), ("/c.png", true, 3)]);
    }

    #[test]
    fn future_dates_on_published_documents_are_warned_about() {
        let today: Date = "2026-01-01".parse().unwrap();
        let parse = |extra: &str| {
            Document::parse(PathBuf::from("p.md"), &format!("---\ntitle = \"T\"\ndate = \"2026-06-01\"\n{extra}---\n")).unwrap()
        };

        let lint = check_date(&parse(""), today).unwrap();
        assert_eq!(lint.level, Level::Warning);
        assert!(lint.message.contains("2026-06-01"), "{lint}");

        assert!(check_date(&parse("draft = true\n"), today).is_none());
        assert!(check_date(&parse(""), "2026-06-01".parse().unwrap()).is_none());
    }

    fn lints(markdown: &str) -> Vec<Lint> {
        let path = Path::new("doc.md");
        let mut lints = check_rust_syntax(path, markdown);
        lints.extend(check_code_block_languages(path, markdown));
        lints.extend(check_shortcodes(path, markdown));
        lints
    }

    #[test]
    fn unknown_shortcodes_are_reported_on_their_line() {
        let lints = lints("intro\n\n{{< note >}} {{< mystery >}}\n\n```\n{{< also_mystery >}}\n```\n");

        assert_eq!(lints.len(), 1);
        assert_eq!(lints[0].line, 3);
        assert_eq!(lints[0].level, Level::Warning);
        assert!(lints[0].message.contains("`mystery`"));
    }

    #[test]
    fn finds_fenced_blocks_with_their_source_lines() {
        let md = "intro\n\n```rust\nfn a() {}\n```\n\ntext\n\n```python\npass\n```\n";
        let blocks = code_blocks(md);

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].line, 3);
        assert_eq!(blocks[0].language(), "rust");
        assert_eq!(blocks[1].line, 9);
        assert_eq!(blocks[1].language(), "python");
    }

    #[test]
    fn indented_blocks_are_not_reported() {
        let blocks = code_blocks("text\n\n    not a fence\n");
        assert!(blocks.is_empty());
    }

    #[test]
    fn info_string_splits_into_language_and_attributes() {
        let blocks = code_blocks("```rust,ignore,edition2021\nnonsense\n```\n");
        let block = &blocks[0];

        assert_eq!(block.language(), "rust");
        assert_eq!(
            block.attributes().collect::<Vec<_>>(),
            ["ignore", "edition2021"],
        );
        assert!(block.is_exempt());
    }

    #[test]
    fn valid_rust_produces_no_lints() {
        assert!(lints("```rust\nfn main() { let x = 1; }\n```\n").is_empty());
    }

    #[test]
    fn broken_rust_is_reported_at_the_fence_line() {
        let found = lints("intro\n\n```rust\nfn main( {\n```\n");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].line, 3);
        assert_eq!(found[0].level, Level::Error);
        assert!(found[0].message.contains("does not parse"), "{}", found[0]);
    }

    #[test]
    fn a_bare_statement_is_treated_as_a_main_body() {
        // Not a valid file on its own, but this is how rustdoc snippets read.
        assert!(lints("```rust\nlet x = 1 + 1;\nassert_eq!(x, 2);\n```\n").is_empty());
    }

    #[test]
    fn hidden_lines_are_part_of_the_program() {
        let md = "```rust\n# fn helper() -> u32 { 1 }\nlet x = helper();\n```\n";
        assert!(lints(md).is_empty());
    }

    #[test]
    fn a_doubled_hash_is_a_literal_hash() {
        // `##[derive(..)]` is how an attribute is escaped in a doc example.
        let md = "```rust\n##[derive(Debug)]\nstruct S;\n```\n";
        assert!(lints(md).is_empty(), "{:?}", lints(md));
    }

    #[test]
    fn ignore_exempts_a_block_from_the_syntax_check() {
        assert!(check_rust_syntax(Path::new("d.md"), "```rust,ignore\nthis is prose\n```\n").is_empty());
    }

    #[test]
    fn compile_fail_exempts_a_block_from_the_syntax_check() {
        let md = "```rust,compile_fail\nfn main( {\n```\n";
        assert!(check_rust_syntax(Path::new("d.md"), md).is_empty());
    }

    #[test]
    fn rs_is_accepted_as_a_spelling_of_rust() {
        let found = check_rust_syntax(Path::new("d.md"), "```rs\nfn main( {\n```\n");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn non_rust_blocks_are_not_parsed_as_rust() {
        let md = "```python\ndef f(:\n```\n";
        assert!(check_rust_syntax(Path::new("d.md"), md).is_empty());
    }

    #[test]
    fn an_unhighlightable_language_is_a_warning() {
        let found = check_code_block_languages(Path::new("d.md"), "```rustlang\nfn main() {}\n```\n");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].level, Level::Warning);
    }

    #[test]
    fn a_block_with_no_language_is_not_warned_about() {
        assert!(check_code_block_languages(Path::new("d.md"), "```\nplain\n```\n").is_empty());
    }

    #[test]
    fn lines_are_reported_against_the_file_not_the_content() {
        let raw = "---\ntitle = \"T\"\n---\n\nintro\n\n```rust\nfn main( {\n```\n";
        let doc = Document::parse(PathBuf::from("post.md"), raw).unwrap();

        // The fence is on line 7 of the file and line 3 of the content.
        assert_eq!(raw.lines().nth(6), Some("```rust"));

        let found = check_document(&doc, &context_for(&doc));
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].line, 7, "{}", found[0]);
    }

    #[test]
    fn a_document_without_frontmatter_needs_no_offset() {
        let raw = "intro\n\n```rust\nfn main( {\n```\n";
        let doc = Document::parse(PathBuf::from("post.md"), raw).unwrap();

        assert_eq!(doc.content_line_offset, 0);
        assert_eq!(check_document(&doc, &context_for(&doc))[0].line, 3);
    }

    #[test]
    fn lints_render_with_a_source_location() {
        let lint = Lint {
            path: PathBuf::from("content/post.md"),
            line: 12,
            level: Level::Error,
            message: "broken".to_owned(),
        };

        assert_eq!(lint.to_string(), "content/post.md:12: error: broken");
    }
}
