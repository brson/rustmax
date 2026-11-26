//! Support for The Rust Programming Language's `<Listing>` tags.
//!
//! The book wraps its numbered code listings in a custom tag,
//! `<Listing number="12-4" file-name="src/main.rs" caption="...">`,
//! which its `trpl-listing` preprocessor turns into a figure with the file
//! name above the code and a numbered caption below. Left alone, a browser
//! ignores the unknown tag and the number, file name and caption are lost.
//!
//! This produces the same markup as upstream's preprocessor.

use rmx::regex::Regex;
use std::sync::LazyLock;

use crate::highlight::html_escape;

static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"([a-z-]+)="([^"]*)""#).unwrap());

/// Whether a book is authored against the `trpl-listing` preprocessor.
pub fn is_listing_book(book_toml: &toml::Value) -> bool {
    book_toml
        .get("preprocessor")
        .and_then(|p| p.get("trpl-listing"))
        .is_some()
}

/// Rewrite every `<Listing>` block in a chapter.
///
/// `render_inline` renders a caption, which is itself markdown.
pub fn rewrite(content: &str, render_inline: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(content.len());
    let mut closing = None;

    for line in content.lines() {
        // A listing may sit in a blockquote, in which case everything we emit
        // has to carry the same `>` markers.
        let trimmed = line.trim_start_matches(['>', ' ']).trim_end();
        let quote = line[..line.len() - line.trim_start_matches(['>', ' ']).len()].trim_end();
        let push = |out: &mut String, text: &str| {
            for emitted in text.split('\n') {
                match (quote.is_empty(), emitted.is_empty()) {
                    (true, _) => out.push_str(emitted),
                    (false, true) => out.push_str(quote),
                    (false, false) => {
                        out.push_str(quote);
                        out.push(' ');
                        out.push_str(emitted);
                    }
                }
                out.push('\n');
            }
        };

        if trimmed.starts_with("<Listing") && trimmed.ends_with('>') {
            let mut number = None;
            let mut file_name = None;
            let mut caption = None;
            for caps in ATTR_RE.captures_iter(trimmed) {
                let value = caps[2].to_string();
                match &caps[1] {
                    "number" => number = Some(value),
                    "file-name" => file_name = Some(value),
                    "caption" => caption = Some(value),
                    _ => {}
                }
            }

            let id = number
                .as_ref()
                .map(|n| format!(" id=\"listing-{}\"", html_escape(n)))
                .unwrap_or_default();
            let mut opening = format!("<figure class=\"listing\"{id}>");
            if let Some(file_name) = file_name {
                opening.push_str(&format!(
                    "\n<span class=\"file-name\">Filename: {}</span>",
                    html_escape(&file_name)
                ));
            }
            // A blank line ends the HTML block so the code inside is parsed
            // as markdown.
            opening.push('\n');
            push(&mut out, &opening);

            let caption = caption.map(|c| render_inline(&unescape_attr(&c)));
            closing = Some(match (number, caption) {
                (Some(n), caption) => {
                    let n = html_escape(&n);
                    let caption = caption.map(|c| format!(": {c}")).unwrap_or_default();
                    format!("<figcaption><a href=\"#listing-{n}\">Listing {n}</a>{caption}</figcaption>\n</figure>")
                }
                (None, Some(caption)) => format!("<figcaption>{caption}</figcaption>\n</figure>"),
                (None, None) => "</figure>".to_string(),
            });
        } else if trimmed == "</Listing>" {
            match closing.take() {
                Some(closing) => push(&mut out, &format!("\n{closing}")),
                None => {
                    eprintln!("  Warning: `</Listing>` without an opening tag");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

fn unescape_attr(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Styling for the elements this module emits.
pub const CSS: &str = r#"
figure.listing { margin: 1rem 0; }
figure.listing .file-name { font: var(--rmx-font-code); font-size: 0.85em; }
figure.listing pre { margin: 0.25rem 0; }
figure.listing figcaption { font-size: 0.9em; font-style: italic; }
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn render(md: &str) -> String {
        rewrite(md, |c| c.replace('`', "|"))
    }

    #[test]
    fn test_listing_becomes_a_figure() {
        let out = render(
            "<Listing number=\"12-4\" file-name=\"src/main.rs\" caption=\"Reading `x`\">\n\n```rust\nfn main() {}\n```\n\n</Listing>\n",
        );
        assert!(out.starts_with("<figure class=\"listing\" id=\"listing-12-4\">\n<span class=\"file-name\">Filename: src/main.rs</span>\n\n"), "{out}");
        assert!(out.contains("```rust\nfn main() {}\n```"), "{out}");
        assert!(out.contains("<figcaption><a href=\"#listing-12-4\">Listing 12-4</a>: Reading |x|</figcaption>\n</figure>"), "{out}");
    }

    #[test]
    fn test_listing_in_a_blockquote() {
        let out = render("> <Listing file-name=\"src/main.rs\">\n>\n> ```rust\n> x\n> ```\n>\n> </Listing>\n");
        assert_eq!(
            out,
            "> <figure class=\"listing\">\n> <span class=\"file-name\">Filename: src/main.rs</span>\n>\n>\n> ```rust\n> x\n> ```\n>\n>\n> </figure>\n"
        );
    }

    #[test]
    fn test_unnumbered_listing() {
        let out = render("<Listing file-name=\"Cargo.toml\">\n\n```toml\n```\n\n</Listing>\n");
        assert!(out.contains("Filename: Cargo.toml"), "{out}");
        assert!(out.trim_end().ends_with("</figure>"), "{out}");
        assert!(!out.contains("figcaption"), "{out}");
    }
}
