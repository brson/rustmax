//! Support for the Rust Reference's custom markdown syntax.
//!
//! The Reference is authored against its own mdbook preprocessor, which lives
//! at `tools/mdbook-spec` in the `rust-lang/reference` repository. None of the
//! syntax it introduces is markdown, so a plain markdown renderer leaves it in
//! the output as literal text. This module reimplements the parts of that
//! preprocessor that do not need a compiler or a `rust-lang/rust` checkout:
//!
//! - Rule identifiers. Most clauses are preceded by a line like
//!   `r[expr.array.repeat]` naming the rule that clause states. These become
//!   linkable anchors.
//! - Rule links. A rule can be linked to from anywhere in the book by its
//!   identifier, either as a bare `[expr.array.repeat]` or through a link
//!   reference definition whose destination is the identifier.
//! - Admonitions. Blockquotes headed by `> [!NOTE]`, `> [!WARNING]`,
//!   `> [!EXAMPLE]`, or `> [!EDITION-2021]` become styled callouts.
//!
//! Two features are deliberately left out. Grammar blocks (` ```grammar `
//! fences) are rendered as plain code rather than as cross-linked productions
//! and railroad diagrams, because that needs the Reference's grammar parser.
//! Links into the standard library are left unresolved, because upstream
//! resolves them by running rustdoc.

use rmx::regex::{Captures, Regex};
use std::collections::BTreeMap;
use std::sync::LazyLock;

use super::{Book, Chapter};

/// A rule identifier on a line by itself, like `r[expr.array]`.
static RULE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^r\[([^]]+)]$").unwrap());

/// A link reference definition, like `[the label]: some-destination`.
static LINK_DEF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\[(?<label>[^]]+)]: +(?<dest>.*)").unwrap());

/// A blockquote headed by an admonition marker, like `> [!NOTE]`.
static ADMONITION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^ *> \[!(?<kind>[^]]+)\]\n(?<body>(?: *>.*\n)+)").unwrap()
});

/// The rules defined across a book, and where each one lives.
pub struct Spec {
    /// Rule identifier to the page defining it, as a path relative to the
    /// book's source directory with an `.html` extension and `/` separators.
    rules: BTreeMap<String, String>,
}

/// Whether a book is authored against the Reference's `spec` preprocessor.
pub fn is_spec_book(book_toml: &str) -> bool {
    book_toml
        .lines()
        .any(|line| line.trim() == "[preprocessor.spec]")
}

impl Spec {
    /// Find every rule defined in the book and the page that defines it.
    pub fn collect(book: &Book, read_chapter: impl Fn(&Chapter) -> Option<String>) -> Spec {
        let mut rules = BTreeMap::new();

        for chapter in super::flatten_chapters(&book.chapters) {
            let Some(path) = &chapter.path else {
                continue;
            };
            let Some(content) = read_chapter(chapter) else {
                continue;
            };
            let page = path.with_extension("html").display().to_string().replace('\\', "/");
            for caps in RULE_RE.captures_iter(&content) {
                rules.insert(caps[1].to_string(), page.clone());
            }
        }

        Spec { rules }
    }

    /// The link a rule identifier resolves to, relative to the current page.
    ///
    /// `path_to_root` is the `../` prefix that takes the page being rendered
    /// back to the book's root.
    pub fn rule_link(&self, path_to_root: &str, rule_id: &str) -> Option<String> {
        let page = self.rules.get(rule_id)?;
        Some(format!("{path_to_root}{page}#r-{rule_id}"))
    }

    /// Rewrite one chapter's markdown into something a markdown renderer can
    /// make sense of.
    pub fn preprocess(&self, content: &str, path_to_root: &str) -> String {
        let content = admonitions(content);
        let content = self.rule_link_definitions(&content, path_to_root);
        rule_definitions(&content)
    }

    /// Point link reference definitions that name a rule at that rule.
    ///
    /// The Reference writes these as `[zero-sized]: glossary.zst`, where the
    /// destination is a rule identifier rather than a URL.
    fn rule_link_definitions(&self, content: &str, path_to_root: &str) -> String {
        LINK_DEF_RE
            .replace_all(content, |caps: &Captures<'_>| {
                match self.rule_link(path_to_root, &caps["dest"]) {
                    Some(url) => format!("[{}]: {url}", &caps["label"]),
                    None => caps[0].to_string(),
                }
            })
            .into_owned()
    }
}

/// Turn each `r[rule.id]` line into an anchor that can be linked to.
fn rule_definitions(content: &str) -> String {
    RULE_RE
        .replace_all(content, |caps: &Captures<'_>| {
            let rule_id = &caps[1];
            // Rule identifiers are long, so allow the line to break after each
            // component rather than overflowing the column.
            let wrappable = rule_id.replace('.', "<wbr>.");
            format!(
                "<div class=\"rule\" id=\"r-{rule_id}\">\
                 <a class=\"rule-link\" href=\"#r-{rule_id}\" title=\"{rule_id}\">\
                 [{wrappable}]</a></div>\n"
            )
        })
        .into_owned()
}

/// Turn blockquotes headed by `> [!NOTE]` and friends into styled callouts.
fn admonitions(content: &str) -> String {
    ADMONITION_RE
        .replace_all(content, |caps: &Captures<'_>| {
            let kind = caps["kind"].to_lowercase();
            let body = &caps["body"];
            // Admonitions nested in a list are indented; keep that indentation
            // so the callout stays inside the list item.
            let indent = &body[..body.find(|c| c != ' ').unwrap_or(0)];

            let (class, title) = match kind.strip_prefix("edition-") {
                Some(edition) => (
                    "edition",
                    format!(
                        "<span class=\"alert-title-edition\">{edition}</span> Edition differences"
                    ),
                ),
                None => (kind.as_str(), initial_case(&kind)),
            };

            format!(
                "{indent}<div class=\"alert alert-{class}\">\n\
                 \n\
                 {indent}> <p class=\"alert-title\">{title}</p>\n\
                 {indent}>\n\
                 {body}\n\
                 \n\
                 {indent}</div>\n"
            )
        })
        .into_owned()
}

fn initial_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Styling for the elements this module emits.
pub const CSS: &str = r#"
/* The Rust Reference's rule identifiers, from its `r[...]` syntax. The label
   sits in the right margin of the clause it names, so prose flows past it. */
.rule {
    float: right;
    clear: right;
    margin-left: 1rem;
    text-align: right;
    font: var(--rmx-font-code);
    font-size: 0.7rem;
    line-height: 1.7;
    /* We insert `<wbr>` ourselves and only want breaks there. */
    word-break: keep-all;
}

/* Anything that establishes its own block formatting context would be squeezed
   in beside the label rather than flowing past it, so start it below instead. */
h1, h2, h3, h4, h5, h6, pre, table, blockquote, .alert { clear: right; }

.rule-link, .rule-link:hover { color: var(--rmx-color-border); }

/* Mark the rule the page was scrolled to, the way a linked heading is. */
.rule:target .rule-link { color: var(--rmx-color-accents); font-weight: bold; }

/* The Rust Reference's admonitions, from its `> [!NOTE]` syntax. */
.alert-title {
    font: var(--rmx-font-em);
    margin: 0;
    color: var(--rmx-color-accents);
}

.alert > blockquote { border-left-color: var(--rmx-color-border); }
.alert-note > blockquote { border-left-color: var(--rmx-color-links); }
.alert-warning > blockquote { border-left-color: var(--rmx-color-accents); }
.alert-example > blockquote { border-left-style: dashed; }
.alert-edition > blockquote { border-left-style: dotted; }
.alert-title-edition { font: var(--rmx-font-code); }

/* Grammar productions, which we render as plain text rather than as the
   cross-linked railroad diagrams the Reference's own tooling produces. */
code.language-grammar { font-size: 0.85em; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_definition_becomes_an_anchor() {
        let html = rule_definitions("r[expr.array]\nSome prose.\n");
        assert!(html.contains(r#"<div class="rule" id="r-expr.array">"#), "{html}");
        assert!(html.contains(r##"href="#r-expr.array""##), "{html}");
        assert!(html.contains("[expr<wbr>.array]"), "{html}");
        // A blank line has to follow, or the prose is swallowed into the raw
        // HTML block instead of being parsed as markdown.
        assert!(html.contains("</div>\n\nSome prose."), "{html:?}");
    }

    #[test]
    fn test_non_rule_lines_are_left_alone() {
        for line in ["r[not a rule] trailing", "  r[indented]", "x r[mid]"] {
            assert_eq!(rule_definitions(line), line);
        }
    }

    #[test]
    fn test_admonition_becomes_a_callout() {
        let md = admonitions("> [!NOTE]\n> Something to know.\n");
        assert!(md.contains(r#"<div class="alert alert-note">"#), "{md}");
        assert!(md.contains(r#"<p class="alert-title">Note</p>"#), "{md}");
        assert!(md.contains("> Something to know."), "{md}");
    }

    #[test]
    fn test_edition_admonition_names_the_edition() {
        let md = admonitions("> [!EDITION-2021]\n> This changed.\n");
        assert!(md.contains(r#"<div class="alert alert-edition">"#), "{md}");
        assert!(
            md.contains(r#"<span class="alert-title-edition">2021</span> Edition differences"#),
            "{md}"
        );
    }

    #[test]
    fn test_indented_admonition_keeps_its_indentation() {
        let md = admonitions("  > [!WARNING]\n  > Careful.\n");
        assert!(md.starts_with(r#"  <div class="alert alert-warning">"#), "{md}");
    }

    fn spec() -> Spec {
        Spec {
            rules: BTreeMap::from([
                ("expr.array".to_string(), "expressions/array-expr.html".to_string()),
                ("glossary.zst".to_string(), "glossary.html".to_string()),
            ]),
        }
    }

    #[test]
    fn test_rule_links_are_relative_to_the_page() {
        assert_eq!(
            spec().rule_link("", "glossary.zst").unwrap(),
            "glossary.html#r-glossary.zst"
        );
        assert_eq!(
            spec().rule_link("../", "expr.array").unwrap(),
            "../expressions/array-expr.html#r-expr.array"
        );
        assert_eq!(spec().rule_link("", "no.such.rule"), None);
    }

    #[test]
    fn test_link_definitions_naming_a_rule_are_rewritten() {
        let md = spec().rule_link_definitions("[zero-sized]: glossary.zst\n", "../");
        assert_eq!(md, "[zero-sized]: ../glossary.html#r-glossary.zst\n");
    }

    #[test]
    fn test_ordinary_link_definitions_are_left_alone() {
        let md = spec().rule_link_definitions("[array]: ../types/array.md\n", "");
        assert_eq!(md, "[array]: ../types/array.md\n");
    }

    #[test]
    fn test_is_spec_book() {
        assert!(is_spec_book("[book]\ntitle = \"x\"\n\n[preprocessor.spec]\ncommand = \"y\"\n"));
        assert!(!is_spec_book("[book]\ntitle = \"x\"\n"));
        assert!(!is_spec_book("[preprocessor.speculate]\n"));
    }
}
