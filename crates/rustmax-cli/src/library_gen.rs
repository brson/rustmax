use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Books {
    books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    slug: String,
    name: String,
    repo: String,
    #[serde(default)]
    upstream_url: Option<String>,
    #[serde(default)]
    book_path: Option<String>,
    #[serde(default)]
    needs_nightly: bool,
}

pub fn generate_library_page() -> AnyResult<()> {
    let root = std::path::Path::new(".");
    let books = load_books(root)?;
    let template = load_template(root)?;
    let content = render_template(&template, &books)?;
    fs::write("book/src/library.md", content)?;
    println!("Generated library.md with local book links");
    Ok(())
}

fn load_books(root: &std::path::Path) -> AnyResult<Books> {
    let path = root.join("src/books.json5");
    let json = fs::read_to_string(path)?;
    Ok(rmx::json5::from_str(&json)?)
}

fn load_template(root: &std::path::Path) -> AnyResult<String> {
    let path = root.join("src/library-template.md");
    Ok(fs::read_to_string(path)?)
}

/// Check if a book was successfully built and copied to work/library/.
fn book_is_built(slug: &str) -> bool {
    let index_path = format!("work/library/{}/index.html", slug);
    fs::exists(&index_path).unwrap_or(false)
}

fn render_template(template: &str, books: &Books) -> AnyResult<String> {
    let book_map: HashMap<&str, &Book> = books
        .books
        .iter()
        .map(|b| (b.slug.as_str(), b))
        .collect();

    let mut output = String::new();
    let mut lines = template.lines().peekable();

    while let Some(line) = lines.next() {
        if let Some(rendered) = try_render_book_directive(line, &book_map) {
            if !rendered.is_empty() {
                output.push_str(&rendered);
                output.push('\n');
            }
        } else if let Some(slug) = try_parse_if_book_start(line) {
            // Collect lines until {{/if-book}}
            let mut block_lines = Vec::new();
            while let Some(block_line) = lines.next() {
                if block_line.trim() == "{{/if-book}}" {
                    break;
                }
                block_lines.push(block_line);
            }
            // Only output block if book is built
            if book_is_built(&slug) {
                for block_line in block_lines {
                    output.push_str(block_line);
                    output.push('\n');
                }
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    // Remove trailing newline to match original behavior, then add one back
    Ok(output.trim_end().to_string() + "\n")
}

/// Try to parse and render a `{{book:slug}}` or `{{book:slug:bold}}` directive.
/// Returns Some(rendered_line) if this line is a book directive, None otherwise.
fn try_render_book_directive(line: &str, book_map: &HashMap<&str, &Book>) -> Option<String> {
    let trimmed = line.trim();

    // Check for {{book:slug}} or {{book:slug:bold}}
    if !trimmed.starts_with("{{book:") || !trimmed.ends_with("}}") {
        return None;
    }

    let inner = &trimmed[7..trimmed.len() - 2]; // Strip {{book: and }}
    let parts: Vec<&str> = inner.split(':').collect();

    let slug = parts[0];
    let bold = parts.get(1).map(|s| *s == "bold").unwrap_or(false);

    // Only render if book is built
    let book = book_map.get(slug).filter(|_| book_is_built(slug))?;

    let upstream_link = book.upstream_url.as_ref().unwrap_or(&book.repo);

    if bold {
        Some(format!(
            "- **[{}](../library/{}/)** ([upstream]({}))",
            book.name, book.slug, upstream_link
        ))
    } else {
        Some(format!(
            "- [{}](../library/{}/) ([upstream]({}))",
            book.name, book.slug, upstream_link
        ))
    }
}

/// Try to parse `{{#if-book:slug}}` and return the slug if matched.
fn try_parse_if_book_start(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("{{#if-book:") && trimmed.ends_with("}}") {
        let slug = &trimmed[11..trimmed.len() - 2];
        Some(slug.to_string())
    } else {
        None
    }
}
