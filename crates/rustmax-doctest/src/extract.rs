use rmx::prelude::*;
use std::fs;
use std::path::Path;
use pulldown_cmark::{Event, Parser, Tag, CodeBlockKind, HeadingLevel};
use crate::types::DocTest;

/// Extract all doctest examples from markdown files in a directory.
pub fn extract_all_examples(doc_dir: &Path) -> AnyResult<Vec<DocTest>> {
    let mut all_examples = Vec::new();

    for entry in fs::read_dir(doc_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let examples = extract_examples_from_file(&path)?;
            all_examples.extend(examples);
        }
    }

    Ok(all_examples)
}

/// Extract doctest examples from a single markdown file.
fn extract_examples_from_file(md_path: &Path) -> AnyResult<Vec<DocTest>> {
    let content = fs::read_to_string(md_path)?;
    let parser = Parser::new(&content);

    let mut examples = Vec::new();
    let mut in_examples_section = false;
    let mut example_count = 0;
    let mut pending_code: Option<(String, bool, bool)> = None;
    let mut check_for_examples_text = false;

    // Estimate line numbers (approximate).
    let mut line = 1;

    for event in parser {
        match &event {
            Event::Start(Tag::Heading { level: HeadingLevel::H2, .. }) => {
                check_for_examples_text = true;
                in_examples_section = false;
            }
            Event::Text(text) => {
                if check_for_examples_text && text.trim() == "Examples" {
                    in_examples_section = true;
                    check_for_examples_text = false;
                } else {
                    check_for_examples_text = false;
                }

                if pending_code.is_some() {
                    if let Some((ref mut code, _, _)) = pending_code {
                        code.push_str(text.as_ref());
                    }
                }
            }
            Event::Code(code_text) => {
                if pending_code.is_some() {
                    if let Some((ref mut code, _, _)) = pending_code {
                        code.push_str(code_text.as_ref());
                    }
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                if in_examples_section {
                    let info_str = info.as_ref();
                    if info_str.starts_with("rust") || info_str.is_empty() {
                        let (no_run, ignore) = parse_modifiers(info_str);
                        pending_code = Some((String::new(), no_run, ignore));
                    }
                }
            }
            Event::End(tag) => {
                if matches!(tag, pulldown_cmark::TagEnd::CodeBlock) {
                    if let Some((code, no_run, ignore)) = pending_code.take() {
                        example_count += 1;
                        let name = generate_test_name(md_path, example_count);

                        examples.push(DocTest {
                            source_file: md_path.to_path_buf(),
                            line,
                            name,
                            code,
                            no_run,
                            ignore,
                        });
                    }
                }
            }
            _ => {}
        }

        // Very rough line tracking.
        if matches!(&event, Event::HardBreak | Event::End(_)) {
            line += 1;
        }
    }

    Ok(examples)
}

/// Parse modifiers from fence info string.
///
/// Examples: "rust,no_run" -> (true, false), "rust,ignore" -> (false, true)
fn parse_modifiers(info: &str) -> (bool, bool) {
    let parts: Vec<&str> = info.split(',').map(|s| s.trim()).collect();

    let no_run = parts.contains(&"no_run");
    let ignore = parts.contains(&"ignore");

    (no_run, ignore)
}

/// Generate a test name from the markdown file and example number.
fn generate_test_name(md_path: &Path, example_num: usize) -> String {
    let filename = md_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Sanitize filename for use as identifier.
    let sanitized = filename
        .replace('-', "_")
        .replace('.', "_");

    format!("{}_{:03}", sanitized, example_num)
}
