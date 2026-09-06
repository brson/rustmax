//! Content lints reported by `anthology check`.
//!
//! Anthology is aimed at technical writing, where most of what goes wrong in a
//! document is not the prose but the code around it. These lints read the
//! parsed markdown rather than the rendered HTML, so they can point at the
//! source line the author wrote.

use rmx::comrak::nodes::NodeValue;
use rmx::comrak::{Arena, Options, parse_document};
use rmx::prelude::*;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::collection::Document;

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
/// Anthology can highlight.
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
                "code block language `{}` is not highlighted; it renders as plain text",
                block.language(),
            ),
        })
        .collect()
}

/// Run every lint against a document.
///
/// Lines are reported against the source file, frontmatter included, so they
/// match what the author sees in an editor.
pub fn check_document(doc: &Document) -> Vec<Lint> {
    let path = &doc.source_path;
    let mut lints = check_rust_syntax(path, &doc.content);
    lints.extend(check_code_block_languages(path, &doc.content));

    for lint in &mut lints {
        lint.line = doc.source_line(lint.line);
    }

    lints.sort_by_key(|lint| lint.line);
    lints
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lints(markdown: &str) -> Vec<Lint> {
        let path = Path::new("doc.md");
        let mut lints = check_rust_syntax(path, markdown);
        lints.extend(check_code_block_languages(path, markdown));
        lints
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

        let found = check_document(&doc);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].line, 7, "{}", found[0]);
    }

    #[test]
    fn a_document_without_frontmatter_needs_no_offset() {
        let raw = "intro\n\n```rust\nfn main( {\n```\n";
        let doc = Document::parse(PathBuf::from("post.md"), raw).unwrap();

        assert_eq!(doc.content_line_offset, 0);
        assert_eq!(check_document(&doc)[0].line, 3);
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
