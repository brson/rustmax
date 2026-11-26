//! The Rust Reference's grammar productions.
//!
//! The Reference states its syntax in ` ```grammar,<category> ` blocks of
//! productions like `EnumVariants -> EnumVariant ( `,` EnumVariant )* `,`?`.
//! Upstream parses these and renders each as text and as a railroad diagram,
//! with every nonterminal linked to its production.
//!
//! We render the text form only. Each production gets an anchor, each name
//! that is a production elsewhere links to it, and the grammar summary page,
//! which upstream generates, lists every production by category.

use rmx::regex::Regex;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::LazyLock;

use crate::highlight::html_escape;

/// A grammar block, capturing its category and body.
static BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?ms)^```grammar,([^\n]+)\n(.*?)^```[ \t]*$").unwrap());

/// The first line of a production, capturing its name.
static PRODUCTION_START_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:@root\s+)?(\S+)\s*->").unwrap());

/// The marker on the grammar summary page where upstream inserts every production.
const SUMMARY_MARKER: &str = "{{ grammar-summary }}";

/// One production, and where it is defined.
pub struct Production {
    /// The page defining it, relative to the book root.
    pub page: String,
    pub category: String,
    /// The production as written.
    pub text: String,
}

/// Every production in the book.
#[derive(Default)]
pub struct Grammar {
    pub productions: BTreeMap<String, Production>,
    /// Production names in the order they appear in the book.
    pub order: Vec<String>,
}

impl Grammar {
    /// Record the productions defined on one page.
    pub fn collect_page(&mut self, page: &str, content: &str) {
        for block in BLOCK_RE.captures_iter(content) {
            let category = block[1].trim().to_string();
            for (name, text) in productions(&block[2]) {
                self.order.push(name.clone());
                self.productions.insert(
                    name,
                    Production { page: page.to_string(), category: category.clone(), text },
                );
            }
        }
    }

    /// Where a production's anchor is, relative to the current page.
    pub fn link(&self, path_to_root: &str, name: &str, on_summary: bool) -> Option<String> {
        let production = self.productions.get(name)?;
        Some(if on_summary {
            format!("#{}", summary_id(name))
        } else {
            format!("{path_to_root}{}#{}", production.page, id(name))
        })
    }

    /// Render the grammar blocks of one page, fill in the summary page, and
    /// make every production linkable as `[Name]`.
    pub fn preprocess(&self, content: &str, path_to_root: &str) -> String {
        if self.productions.is_empty() {
            return content.to_string();
        }
        let on_summary = content.contains(SUMMARY_MARKER);

        let mut content = BLOCK_RE
            .replace_all(content, |caps: &rmx::regex::Captures<'_>| {
                self.render_block(&caps[2], path_to_root, on_summary)
            })
            .into_owned();

        if on_summary {
            content = content.replace(SUMMARY_MARKER, &self.summary(path_to_root));
        }

        // A link reference definition a page writes itself comes first and
        // wins over these.
        content.push('\n');
        for name in self.productions.keys() {
            let link = self.link(path_to_root, name, on_summary).expect("a known production");
            let _ = writeln!(content, "[{name}]: {link}\n[grammar-{name}]: {link}");
        }
        content
    }

    fn summary(&self, path_to_root: &str) -> String {
        let mut categories: Vec<&str> = Vec::new();
        for name in &self.order {
            let category = self.productions[name].category.as_str();
            if !categories.contains(&category) {
                categories.push(category);
            }
        }

        let mut out = String::new();
        for category in categories {
            let mut chars = category.chars();
            let title = chars.next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default()
                + chars.as_str();
            let _ = write!(out, "\n## {title} summary\n\n");
            let body: Vec<&str> = self
                .order
                .iter()
                .filter(|name| self.productions[*name].category == category)
                .map(|name| self.productions[name].text.as_str())
                .collect();
            out.push_str(&self.render_block(&body.join("\n\n"), path_to_root, true));
            out.push('\n');
        }
        out
    }

    /// A grammar block as a `<pre>` of anchored, cross-linked productions.
    fn render_block(&self, body: &str, path_to_root: &str, on_summary: bool) -> String {
        let mut html = String::from("<pre class=\"grammar\"><code>");
        for line in body.lines() {
            let rest = match PRODUCTION_START_RE.captures(line) {
                Some(caps) => {
                    let name = caps.get(1).expect("group 1 always matches");
                    let anchor = if on_summary { summary_id(name.as_str()) } else { id(name.as_str()) };
                    html.push_str(&html_escape(&line[..name.start()]));
                    let _ = write!(
                        html,
                        "<span class=\"grammar-production\" id=\"{anchor}\">{}</span>",
                        html_escape(name.as_str())
                    );
                    &line[name.end()..]
                }
                None => line,
            };
            self.render_tokens(rest, path_to_root, on_summary, &mut html);
            html.push('\n');
        }
        // A blank line keeps whatever follows from being parsed as part of
        // the HTML block.
        html.push_str("</code></pre>\n");
        html
    }

    fn render_tokens(&self, text: &str, path_to_root: &str, on_summary: bool, html: &mut String) {
        let mut rest = text;
        while let Some(c) = rest.chars().next() {
            if c == '`' {
                // A literal runs to the next backtick.
                let end = rest[1..].find('`').map(|i| i + 2).unwrap_or(rest.len());
                let _ = write!(html, "<span class=\"grammar-literal\">{}</span>", html_escape(&rest[..end]));
                rest = &rest[end..];
            } else if c.is_alphabetic() || c == '_' {
                let end = rest
                    .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .unwrap_or(rest.len());
                let word = &rest[..end];
                match self.link(path_to_root, word, on_summary) {
                    Some(link) => {
                        let _ = write!(html, "<a href=\"{}\">{}</a>", html_escape(&link), html_escape(word));
                    }
                    None => html.push_str(&html_escape(word)),
                }
                rest = &rest[end..];
            } else {
                html.push_str(&html_escape(&rest[..c.len_utf8()]));
                rest = &rest[c.len_utf8()..];
            }
        }
    }
}

fn id(name: &str) -> String {
    format!("grammar-{name}")
}

fn summary_id(name: &str) -> String {
    format!("grammar-summary-{name}")
}

/// Split a grammar block into its productions: each starts at a line naming
/// it and runs on through indented continuation lines.
fn productions(body: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in body.lines() {
        if let Some(caps) = PRODUCTION_START_RE.captures(line) {
            out.push((caps[1].to_string(), line.to_string()));
        } else if !line.trim().is_empty()
            && line.starts_with(char::is_whitespace)
            && let Some((_, text)) = out.last_mut()
        {
            text.push('\n');
            text.push_str(line);
        }
    }
    out
}

/// Styling for the elements this module emits.
pub const CSS: &str = r#"
pre.grammar { font-size: 0.85em; }
pre.grammar .grammar-production { font-weight: bold; color: var(--rmx-color-accents); }
pre.grammar .grammar-literal { color: var(--rmx-color-links); opacity: 0.85; }
pre.grammar a { color: inherit; text-decoration: underline dotted; }
pre.grammar :target { background: var(--rmx-color-bg); outline: 1px dashed var(--rmx-color-accents); }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "Text.\n\n```grammar,items\nEnumVariants -> EnumVariant ( `,` EnumVariant )* `,`?\n\nEnumVariant ->\n    OuterAttribute* IDENTIFIER\n```\n\nMore [EnumVariant].\n";

    fn grammar() -> Grammar {
        let mut grammar = Grammar::default();
        grammar.collect_page("items/enumerations.html", PAGE);
        grammar
    }

    #[test]
    fn test_productions_are_collected() {
        let g = grammar();
        assert_eq!(g.order, ["EnumVariants", "EnumVariant"]);
        assert_eq!(g.productions["EnumVariant"].text, "EnumVariant ->\n    OuterAttribute* IDENTIFIER");
        assert_eq!(g.productions["EnumVariant"].category, "items");
    }

    #[test]
    fn test_blocks_get_anchors_and_links() {
        let out = grammar().preprocess(PAGE, "../");
        assert!(out.contains(r#"<span class="grammar-production" id="grammar-EnumVariants">EnumVariants</span>"#), "{out}");
        assert!(out.contains(r#"<a href="../items/enumerations.html#grammar-EnumVariant">EnumVariant</a>"#), "{out}");
        assert!(out.contains(r#"<span class="grammar-literal">`,`</span>"#), "{out}");
        assert!(out.contains("OuterAttribute* IDENTIFIER"), "{out}");
        assert!(out.contains("[EnumVariant]: ../items/enumerations.html#grammar-EnumVariant\n"), "{out}");
        assert!(!out.contains("```grammar"), "{out}");
    }

    #[test]
    fn test_summary_page_lists_every_production() {
        let out = grammar().preprocess("# Grammar summary\n\n{{ grammar-summary }}\n", "");
        assert!(out.contains("## Items summary"), "{out}");
        assert!(out.contains(r#"id="grammar-summary-EnumVariant""#), "{out}");
        assert!(out.contains("[EnumVariant]: #grammar-summary-EnumVariant\n"), "{out}");
    }
}
