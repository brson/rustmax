//! Markdown to HTML rendering with syntax highlighting.

use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use comrak::adapters::SyntaxHighlighterAdapter;
use rmx::regex::Regex;

use super::highlight::Highlighter;
use crate::GlobalItemIndex;

/// Render markdown to HTML with syntax highlighting.
pub fn render_markdown(md: &str, highlighter: &Highlighter) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true; // Allow raw HTML in docs.

    let adapter = HighlightAdapter { highlighter };
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(md, &options, &plugins)
}

/// Render markdown to HTML and resolve intra-doc links using the global index.
pub fn render_markdown_with_links(
    md: &str,
    highlighter: &Highlighter,
    global_index: Option<&GlobalItemIndex>,
    current_crate: &str,
    current_depth: usize,
) -> String {
    // Pre-process markdown to convert rustdoc-style intra-doc links.
    let processed_md = preprocess_intra_doc_links(md);

    let html = render_markdown(&processed_md, highlighter);

    if let Some(index) = global_index {
        resolve_intra_doc_links(&html, index, current_crate, current_depth)
    } else {
        html
    }
}

/// Pre-process markdown to convert rustdoc-style intra-doc links to standard links.
///
/// Converts `[`path::to::Item`]` to `[`path::to::Item`](path::to::Item)`.
/// Also handles `[`::crate`]` syntax for crate-root paths.
fn preprocess_intra_doc_links(md: &str) -> String {
    // Match [`path::to::Item`] or [`::crate`] and capture what follows.
    // We can't use lookahead in Rust regex, so capture trailing context.
    // The (::)? optionally matches the leading :: for crate-root paths.
    let re = Regex::new(
        r#"\[`((::)?[a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)`\](\(|\[)?"#
    ).unwrap();

    re.replace_all(md, |caps: &rmx::regex::Captures| {
        let path = &caps[1];
        // Capture group 3 is the trailing ( or [ if present (group 2 is the optional ::).
        let following = caps.get(3).map(|m| m.as_str());

        match following {
            // Already has a link target or reference, leave it alone.
            Some("(") => format!("[`{}`](", path),
            Some("[") => format!("[`{}`][", path),
            // Bare link, convert to explicit link.
            None => format!("[`{}`]({})", path, path),
            _ => caps[0].to_string(),
        }
    }).into_owned()
}

/// Resolve intra-doc links in HTML output.
///
/// Looks for `<a href="path::to::Item">` or `<a href="::crate">` patterns and resolves them to URLs.
fn resolve_intra_doc_links(
    html: &str,
    global_index: &GlobalItemIndex,
    current_crate: &str,
    current_depth: usize,
) -> String {
    // Match href attributes that look like Rust paths (not URLs, not .html files).
    // Also match paths starting with :: for crate-root references.
    let re = Regex::new(r#"href="((::)?[a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)""#).unwrap();

    re.replace_all(html, |caps: &rmx::regex::Captures| {
        let path = &caps[1];

        // Try to resolve the path.
        if let Some(url) = resolve_path(path, global_index, current_crate, current_depth) {
            format!("href=\"{}\"", url)
        } else {
            // Keep original if can't resolve.
            caps[0].to_string()
        }
    }).into_owned()
}

/// Try to resolve a Rust path to a URL.
fn resolve_path(
    path: &str,
    global_index: &GlobalItemIndex,
    current_crate: &str,
    current_depth: usize,
) -> Option<String> {
    // Strip leading :: for crate-root paths (e.g., ::axum -> axum).
    let path = path.strip_prefix("::").unwrap_or(path);

    // Skip common non-item words.
    if matches!(path, "todo" | "http" | "https" | "mailto" | "ftp" | "file") {
        return None;
    }

    // Build path to root.
    let path_to_root = if current_depth == 0 {
        String::new()
    } else {
        "../".repeat(current_depth)
    };

    // Try exact path first.
    if let Some(location) = global_index.items.get(path) {
        return Some(build_url(&location.path, location.kind, &path_to_root));
    }

    // Try with current crate prefix.
    let with_crate = format!("{}::{}", current_crate, path);
    if let Some(location) = global_index.items.get(&with_crate) {
        return Some(build_url(&location.path, location.kind, &path_to_root));
    }

    // For paths like "bytes::Bytes", try the first component as the crate name.
    let parts: Vec<&str> = path.split("::").collect();
    if parts.len() >= 2 {
        // The first component might be a crate name.
        let potential_crate = parts[0];
        // Try exact match with that crate.
        if let Some(location) = global_index.items.get(path) {
            return Some(build_url(&location.path, location.kind, &path_to_root));
        }
        // Try crate::item (for re-exported items).
        let crate_item = format!("{}::{}", potential_crate, parts[parts.len() - 1]);
        if let Some(location) = global_index.items.get(&crate_item) {
            return Some(build_url(&location.path, location.kind, &path_to_root));
        }
    }

    // For simple names like "Bytes", prefer matches from crates that match the context.
    // Only do fuzzy matching for short paths (likely type names, not modules).
    if parts.len() == 1 && path.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        let simple_name = path;
        // Prefer a direct crate::Item match over deeply nested items.
        let mut best_match: Option<&crate::ItemLocation> = None;
        let mut best_depth = usize::MAX;

        for (full_path, location) in &global_index.items {
            if full_path.ends_with(&format!("::{}", simple_name)) {
                let match_depth = full_path.matches("::").count();
                // Prefer shorter paths (direct exports).
                if match_depth < best_depth {
                    best_match = Some(location);
                    best_depth = match_depth;
                }
            }
        }

        if let Some(location) = best_match {
            return Some(build_url(&location.path, location.kind, &path_to_root));
        }
    }

    None
}

/// Build a URL for an item.
fn build_url(path: &[String], kind: rustdoc_types::ItemKind, path_to_root: &str) -> String {
    use rustdoc_types::ItemKind;

    let kind_prefix = match kind {
        ItemKind::Struct => "struct.",
        ItemKind::Enum => "enum.",
        ItemKind::Trait => "trait.",
        ItemKind::Function => "fn.",
        ItemKind::TypeAlias => "type.",
        ItemKind::Constant => "constant.",
        ItemKind::Static => "static.",
        ItemKind::Macro => "macro.",
        ItemKind::Module => "",
        _ => return format!("{}{}", path_to_root, path.join("/")),
    };

    if path.is_empty() {
        return format!("{}index.html", path_to_root);
    }

    let (dir_parts, name) = path.split_at(path.len() - 1);
    let mut url = path_to_root.to_string();
    for part in dir_parts {
        url.push_str(part);
        url.push('/');
    }

    if kind == ItemKind::Module {
        url.push_str(&name[0]);
        url.push_str("/index.html");
    } else {
        url.push_str(kind_prefix);
        url.push_str(&name[0]);
        url.push_str(".html");
    }

    url
}

struct HighlightAdapter<'a> {
    highlighter: &'a Highlighter,
}

impl SyntaxHighlighterAdapter for HighlightAdapter<'_> {
    fn write_highlighted(
        &self,
        output: &mut dyn std::io::Write,
        lang: Option<&str>,
        code: &str,
    ) -> std::io::Result<()> {
        let lang = lang.unwrap_or("rust");
        let highlighted = self.highlighter.highlight(code, lang);
        write!(output, "{}", highlighted)
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn std::io::Write,
        attributes: std::collections::HashMap<String, String>,
    ) -> std::io::Result<()> {
        let mut attrs = String::new();
        for (key, value) in &attributes {
            attrs.push_str(&format!(" {}=\"{}\"", key, html_escape(value)));
        }
        write!(output, "<pre class=\"highlight\"{}>", attrs)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn std::io::Write,
        attributes: std::collections::HashMap<String, String>,
    ) -> std::io::Result<()> {
        let lang = attributes.get("class")
            .and_then(|c| c.strip_prefix("language-"))
            .unwrap_or("rust");
        write!(output, "<code class=\"language-{}\">", html_escape(lang))
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_simple_intra_doc_link() {
        let md = "See [`Item`] for more info.";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "See [`Item`](Item) for more info.");
    }

    #[test]
    fn test_preprocess_qualified_path() {
        let md = "Use [`std::io::Error`] here.";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "Use [`std::io::Error`](std::io::Error) here.");
    }

    #[test]
    fn test_preprocess_multiple_links() {
        let md = "Both [`Foo`] and [`Bar`] are useful.";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "Both [`Foo`](Foo) and [`Bar`](Bar) are useful.");
    }

    #[test]
    fn test_preprocess_preserves_existing_links() {
        let md = "See [`Item`](https://example.com) for details.";
        let result = preprocess_intra_doc_links(md);
        // Should not double-link.
        assert_eq!(result, "See [`Item`](https://example.com) for details.");
    }

    #[test]
    fn test_preprocess_preserves_reference_links() {
        let md = "See [`Item`][ref] for details.\n\n[ref]: https://example.com";
        let result = preprocess_intra_doc_links(md);
        // Should not transform reference-style links.
        assert_eq!(result, "See [`Item`][ref] for details.\n\n[ref]: https://example.com");
    }

    #[test]
    fn test_preprocess_inline_code_not_in_brackets() {
        let md = "Use `Item` here.";
        let result = preprocess_intra_doc_links(md);
        // Plain inline code without brackets should not be linked.
        assert_eq!(result, "Use `Item` here.");
    }

    #[test]
    fn test_preprocess_crate_path() {
        let md = "See [`crate::module::Type`] for info.";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "See [`crate::module::Type`](crate::module::Type) for info.");
    }

    #[test]
    fn test_preprocess_crate_root_path() {
        let md = "See crate [`::axum`].";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "See crate [`::axum`](::axum).");
    }

    #[test]
    fn test_preprocess_crate_root_qualified() {
        let md = "Use [`::tokio::sync::Mutex`] for async.";
        let result = preprocess_intra_doc_links(md);
        assert_eq!(result, "Use [`::tokio::sync::Mutex`](::tokio::sync::Mutex) for async.");
    }

    #[test]
    fn test_build_url_struct() {
        use rustdoc_types::ItemKind;
        let path = vec!["mycrate".to_string(), "MyStruct".to_string()];
        let url = build_url(&path, ItemKind::Struct, "");
        assert_eq!(url, "mycrate/struct.MyStruct.html");
    }

    #[test]
    fn test_build_url_struct_with_depth() {
        use rustdoc_types::ItemKind;
        let path = vec!["mycrate".to_string(), "MyStruct".to_string()];
        let url = build_url(&path, ItemKind::Struct, "../");
        assert_eq!(url, "../mycrate/struct.MyStruct.html");
    }

    #[test]
    fn test_build_url_module() {
        use rustdoc_types::ItemKind;
        let path = vec!["mycrate".to_string(), "submod".to_string()];
        let url = build_url(&path, ItemKind::Module, "");
        assert_eq!(url, "mycrate/submod/index.html");
    }

    #[test]
    fn test_build_url_trait() {
        use rustdoc_types::ItemKind;
        let path = vec!["std".to_string(), "io".to_string(), "Read".to_string()];
        let url = build_url(&path, ItemKind::Trait, "../../");
        assert_eq!(url, "../../std/io/trait.Read.html");
    }

    #[test]
    fn test_build_url_function() {
        use rustdoc_types::ItemKind;
        let path = vec!["mycrate".to_string(), "my_func".to_string()];
        let url = build_url(&path, ItemKind::Function, "");
        assert_eq!(url, "mycrate/fn.my_func.html");
    }
}
