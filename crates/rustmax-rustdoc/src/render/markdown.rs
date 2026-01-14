//! Markdown to HTML rendering with syntax highlighting and intra-doc link resolution.

use comrak::{markdown_to_html_with_plugins, Arena, Options, Plugins};
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::NodeValue;
use rustdoc_types::ItemKind;

use super::highlight::Highlighter;
use crate::{GlobalItemIndex, ItemLocation};

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

/// Render markdown to HTML with intra-doc link resolution.
///
/// Resolves Rust path references like `` [`Vec`] `` or `[text](std::option::Option)`
/// to actual HTML documentation URLs.
pub fn render_markdown_with_links(
    md: &str,
    highlighter: &Highlighter,
    global_index: Option<&GlobalItemIndex>,
    current_crate: &str,
    current_depth: usize,
) -> String {
    let Some(index) = global_index else {
        return render_markdown(md, highlighter);
    };

    // Preprocess: convert rustdoc shortcut links to explicit markdown links.
    let processed = preprocess_shortcut_links(md);

    // Parse to AST.
    let arena = Arena::new();
    let options = markdown_options();
    let root = comrak::parse_document(&arena, &processed, &options);

    // Walk AST and resolve intra-doc links.
    for node in root.descendants() {
        let mut data = node.data.borrow_mut();
        if let NodeValue::Link(ref mut link) = data.value {
            if is_rust_path(&link.url) {
                if let Some(resolved) = resolve_rust_path(&link.url, index, current_crate, current_depth) {
                    link.url = resolved;
                } else {
                    eprintln!("warning: unresolved intra-doc link: {}", link.url);
                }
            }
        }
    }

    // Render to HTML with syntax highlighting.
    let adapter = HighlightAdapter { highlighter };
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let mut html = Vec::new();
    comrak::format_html_with_plugins(root, &options, &mut html, &plugins).unwrap();
    String::from_utf8_lossy(&html).into_owned()
}

/// Create standard markdown options.
fn markdown_options() -> Options<'static> {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true;
    options
}

/// Preprocess markdown to convert rustdoc shortcut links to explicit links.
///
/// Converts `` [`path::Item`] `` to `` [`path::Item`](path::Item) ``
/// but only if there's no reference definition for that label in the document.
fn preprocess_shortcut_links(md: &str) -> String {
    use rmx::regex::Regex;
    use std::collections::HashSet;

    // First, find all reference definitions in the document.
    // These look like: [`Label`]: url or [Label]: url at the start of a line.
    // Match both forms separately since regex doesn't support backreferences.
    let ref_def_backtick = Regex::new(r#"(?m)^\s*\[`([^`]+)`\]:\s*"#).unwrap();
    let ref_def_plain = Regex::new(r#"(?m)^\s*\[([^\]`]+)\]:\s*"#).unwrap();

    let mut defined_refs: HashSet<String> = HashSet::new();
    for cap in ref_def_backtick.captures_iter(md) {
        defined_refs.insert(cap.get(1).unwrap().as_str().to_string());
    }
    for cap in ref_def_plain.captures_iter(md) {
        defined_refs.insert(cap.get(1).unwrap().as_str().to_string());
    }

    // Match [`path::to::Item`] - we'll check for trailing ( or [ manually.
    let re = Regex::new(
        r#"\[`([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)`\]"#
    ).unwrap();

    let mut result = String::with_capacity(md.len());
    let mut last_end = 0;

    for cap in re.captures_iter(md) {
        let full_match = cap.get(0).unwrap();
        let path = &cap[1];

        // Check if followed by ( or [ - if so, it's already a link.
        let next_char = md[full_match.end()..].chars().next();
        let is_already_link = matches!(next_char, Some('(') | Some('['));

        // Check if followed by : - it's a reference definition.
        let is_ref_definition = matches!(next_char, Some(':'));

        // Check if there's a reference definition for this label.
        // Note: In CommonMark, [`Foo`] and [Foo] are different labels, but rustdoc
        // treats them equivalently, so we check both.
        let has_ref_definition = defined_refs.contains(path);

        result.push_str(&md[last_end..full_match.start()]);
        if is_already_link || is_ref_definition || has_ref_definition {
            result.push_str(full_match.as_str());
        } else {
            result.push_str(&format!("[`{}`]({})", path, path));
        }
        last_end = full_match.end();
    }

    result.push_str(&md[last_end..]);
    result
}

/// Check if a string looks like a Rust path (not a URL).
fn is_rust_path(s: &str) -> bool {
    // URLs have slashes, Rust paths don't.
    if s.contains('/') || s.contains('#') {
        return false;
    }
    // Empty or whitespace is not a path.
    if s.is_empty() || s.chars().all(|c| c.is_whitespace()) {
        return false;
    }
    // Skip bare keywords and literals that can't be resolved.
    if matches!(s, "self" | "true" | "false" | "None" | "Some") {
        return false;
    }
    // Must start with identifier char or disambiguator.
    let path = strip_disambiguator(s).1;
    if path.is_empty() {
        return false;
    }
    // Check it matches Rust identifier pattern.
    let first = path.chars().next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' && first != ':' {
        return false;
    }
    // Valid chars: alphanumeric, underscore, colon.
    path.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
}

/// Parse and strip disambiguator prefix from a path.
///
/// Returns (optional ItemKind constraint, remaining path).
fn strip_disambiguator(s: &str) -> (Option<ItemKind>, &str) {
    if let Some(rest) = s.strip_prefix("struct@") {
        return (Some(ItemKind::Struct), rest);
    }
    if let Some(rest) = s.strip_prefix("enum@") {
        return (Some(ItemKind::Enum), rest);
    }
    if let Some(rest) = s.strip_prefix("trait@") {
        return (Some(ItemKind::Trait), rest);
    }
    if let Some(rest) = s.strip_prefix("union@") {
        return (Some(ItemKind::Union), rest);
    }
    if let Some(rest) = s.strip_prefix("mod@") {
        return (Some(ItemKind::Module), rest);
    }
    if let Some(rest) = s.strip_prefix("module@") {
        return (Some(ItemKind::Module), rest);
    }
    if let Some(rest) = s.strip_prefix("fn@") {
        return (Some(ItemKind::Function), rest);
    }
    if let Some(rest) = s.strip_prefix("function@") {
        return (Some(ItemKind::Function), rest);
    }
    if let Some(rest) = s.strip_prefix("method@") {
        return (Some(ItemKind::Function), rest);
    }
    if let Some(rest) = s.strip_prefix("const@") {
        return (Some(ItemKind::Constant), rest);
    }
    if let Some(rest) = s.strip_prefix("constant@") {
        return (Some(ItemKind::Constant), rest);
    }
    if let Some(rest) = s.strip_prefix("static@") {
        return (Some(ItemKind::Static), rest);
    }
    if let Some(rest) = s.strip_prefix("type@") {
        return (Some(ItemKind::TypeAlias), rest);
    }
    if let Some(rest) = s.strip_prefix("macro@") {
        return (Some(ItemKind::Macro), rest);
    }
    // No disambiguator.
    (None, s)
}

/// Resolve a Rust path to an HTML URL.
fn resolve_rust_path(
    path: &str,
    index: &GlobalItemIndex,
    current_crate: &str,
    current_depth: usize,
) -> Option<String> {
    let (kind_filter, clean_path) = strip_disambiguator(path);

    // Handle crate:: prefix.
    let lookup_path = if let Some(rest) = clean_path.strip_prefix("crate::") {
        format!("{}::{}", current_crate, rest)
    } else if clean_path.starts_with("::") {
        // Absolute path from crate root.
        format!("{}::{}", current_crate, clean_path.trim_start_matches(':'))
    } else {
        clean_path.to_string()
    };

    // For paths without ::, try current crate prefix FIRST.
    // This ensures `anyhow` in rustmax docs resolves to `rustmax::anyhow` (module)
    // rather than `anyhow` (external crate).
    if !lookup_path.contains("::") {
        let crate_prefixed = format!("{}::{}", current_crate, lookup_path);
        if let Some(location) = index.items.get(&crate_prefixed) {
            if matches_kind_filter(location, kind_filter) {
                return Some(build_url(location, current_depth));
            }
        }
    }

    // Try exact match.
    if let Some(location) = index.items.get(&lookup_path) {
        if matches_kind_filter(location, kind_filter) {
            return Some(build_url(location, current_depth));
        }
    }

    // Try with current crate prefix for qualified paths.
    if lookup_path.contains("::") {
        let crate_prefixed = format!("{}::{}", current_crate, lookup_path);
        if let Some(location) = index.items.get(&crate_prefixed) {
            if matches_kind_filter(location, kind_filter) {
                return Some(build_url(location, current_depth));
            }
        }
    }

    // Handle re-exported crate items: rustmax::ahash::AHasher -> ahash::AHasher.
    // When a crate does `pub use some_crate::*;`, the path current_crate::some_crate::Item
    // should resolve to some_crate::Item.
    if let Some(rest) = lookup_path.strip_prefix(&format!("{}::", current_crate)) {
        if let Some(location) = index.items.get(rest) {
            if matches_kind_filter(location, kind_filter) {
                return Some(build_url(location, current_depth));
            }
        }
    }

    // std re-exports most of core, so try core:: when std:: fails.
    if let Some(core_path) = lookup_path.strip_prefix("std::") {
        let core_lookup = format!("core::{}", core_path);
        if let Some(location) = index.items.get(&core_lookup) {
            if matches_kind_filter(location, kind_filter) {
                return Some(build_url(location, current_depth));
            }
        }
    }

    // For single-segment paths, search all items as last resort.
    if !lookup_path.contains("::") {
        for (full_path, location) in &index.items {
            let name = full_path.rsplit("::").next().unwrap_or(full_path);
            if name == lookup_path && matches_kind_filter(location, kind_filter) {
                return Some(build_url(location, current_depth));
            }
        }
    }

    None
}

/// Check if an item location matches the optional kind filter.
fn matches_kind_filter(location: &ItemLocation, filter: Option<ItemKind>) -> bool {
    match filter {
        None => true,
        Some(kind) => location.kind == kind,
    }
}

/// Build an HTML URL from an item location.
fn build_url(location: &ItemLocation, current_depth: usize) -> String {
    let prefix = "../".repeat(current_depth);

    let kind_prefix = match location.kind {
        ItemKind::Struct => "struct.",
        ItemKind::Enum => "enum.",
        ItemKind::Trait => "trait.",
        ItemKind::Function => "fn.",
        ItemKind::TypeAlias => "type.",
        ItemKind::Constant => "constant.",
        ItemKind::Static => "static.",
        ItemKind::Module => "",
        ItemKind::Macro => "macro.",
        ItemKind::Union => "union.",
        _ => "",
    };

    if location.path.len() <= 1 {
        // Crate root.
        format!("{}{}/index.html", prefix, location.crate_name)
    } else {
        let module_path = &location.path[..location.path.len() - 1];
        let item_name = location.path.last().unwrap();

        if matches!(location.kind, ItemKind::Module) {
            // Module pages are at module/index.html.
            format!("{}{}/index.html", prefix, location.path.join("/"))
        } else {
            format!(
                "{}{}/{}{}.html",
                prefix,
                module_path.join("/"),
                kind_prefix,
                item_name
            )
        }
    }
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

    fn test_index() -> GlobalItemIndex {
        let mut index = GlobalItemIndex::default();
        index.items.insert("mycrate::MyStruct".to_string(), ItemLocation {
            crate_name: "mycrate".to_string(),
            path: vec!["mycrate".to_string(), "MyStruct".to_string()],
            kind: ItemKind::Struct,
        });
        index.items.insert("mycrate::module::MyEnum".to_string(), ItemLocation {
            crate_name: "mycrate".to_string(),
            path: vec!["mycrate".to_string(), "module".to_string(), "MyEnum".to_string()],
            kind: ItemKind::Enum,
        });
        index.items.insert("mycrate::MyTrait".to_string(), ItemLocation {
            crate_name: "mycrate".to_string(),
            path: vec!["mycrate".to_string(), "MyTrait".to_string()],
            kind: ItemKind::Trait,
        });
        index.items.insert("mycrate::my_fn".to_string(), ItemLocation {
            crate_name: "mycrate".to_string(),
            path: vec!["mycrate".to_string(), "my_fn".to_string()],
            kind: ItemKind::Function,
        });
        index
    }

    #[test]
    fn test_is_rust_path() {
        assert!(is_rust_path("Foo"));
        assert!(is_rust_path("foo::Bar"));
        assert!(is_rust_path("crate::foo::Bar"));
        assert!(is_rust_path("struct@Foo"));
        assert!(is_rust_path("::crate_root::Item"));

        assert!(!is_rust_path("https://example.com"));
        assert!(!is_rust_path("/path/to/file"));
        assert!(!is_rust_path("foo#section"));
        assert!(!is_rust_path(""));
        assert!(!is_rust_path("   "));
    }

    #[test]
    fn test_strip_disambiguator() {
        assert_eq!(strip_disambiguator("Foo"), (None, "Foo"));
        assert_eq!(strip_disambiguator("struct@Foo"), (Some(ItemKind::Struct), "Foo"));
        assert_eq!(strip_disambiguator("enum@Bar"), (Some(ItemKind::Enum), "Bar"));
        assert_eq!(strip_disambiguator("trait@Baz"), (Some(ItemKind::Trait), "Baz"));
        assert_eq!(strip_disambiguator("fn@func"), (Some(ItemKind::Function), "func"));
        assert_eq!(strip_disambiguator("mod@module"), (Some(ItemKind::Module), "module"));
        assert_eq!(strip_disambiguator("macro@mac"), (Some(ItemKind::Macro), "mac"));
    }

    #[test]
    fn test_preprocess_shortcut_links() {
        assert_eq!(
            preprocess_shortcut_links("See [`Foo`] for details"),
            "See [`Foo`](Foo) for details"
        );
        assert_eq!(
            preprocess_shortcut_links("Link [`foo::Bar`] here"),
            "Link [`foo::Bar`](foo::Bar) here"
        );
        // Should not modify links that already have destinations.
        assert_eq!(
            preprocess_shortcut_links("See [`Foo`](other) here"),
            "See [`Foo`](other) here"
        );
        assert_eq!(
            preprocess_shortcut_links("See [`Foo`][ref] here"),
            "See [`Foo`][ref] here"
        );
        // Should not modify shortcut links that have reference definitions.
        assert_eq!(
            preprocess_shortcut_links("See [`HashMap`] for details.\n\n[`HashMap`]: std::collections::HashMap"),
            "See [`HashMap`] for details.\n\n[`HashMap`]: std::collections::HashMap"
        );
        // Should not modify the reference definition line itself.
        assert_eq!(
            preprocess_shortcut_links("[`Foo`]: some::path"),
            "[`Foo`]: some::path"
        );
        // Should handle backtick mismatch: usage with backticks, definition without.
        assert_eq!(
            preprocess_shortcut_links("See [`HashMap`] for details.\n\n[HashMap]: std::collections::HashMap"),
            "See [`HashMap`] for details.\n\n[HashMap]: std::collections::HashMap"
        );
    }

    #[test]
    fn test_resolve_rust_path_exact() {
        let index = test_index();
        let url = resolve_rust_path("mycrate::MyStruct", &index, "mycrate", 1);
        assert_eq!(url, Some("../mycrate/struct.MyStruct.html".to_string()));
    }

    #[test]
    fn test_resolve_rust_path_crate_prefixed() {
        let index = test_index();
        // Should find MyStruct when looking up just "MyStruct" from within mycrate.
        let url = resolve_rust_path("MyStruct", &index, "mycrate", 1);
        assert_eq!(url, Some("../mycrate/struct.MyStruct.html".to_string()));
    }

    #[test]
    fn test_resolve_rust_path_crate_prefix_syntax() {
        let index = test_index();
        let url = resolve_rust_path("crate::MyStruct", &index, "mycrate", 1);
        assert_eq!(url, Some("../mycrate/struct.MyStruct.html".to_string()));
    }

    #[test]
    fn test_resolve_rust_path_nested_module() {
        let index = test_index();
        let url = resolve_rust_path("mycrate::module::MyEnum", &index, "mycrate", 2);
        assert_eq!(url, Some("../../mycrate/module/enum.MyEnum.html".to_string()));
    }

    #[test]
    fn test_resolve_rust_path_disambiguator() {
        let index = test_index();
        let url = resolve_rust_path("struct@MyStruct", &index, "mycrate", 0);
        assert_eq!(url, Some("mycrate/struct.MyStruct.html".to_string()));

        // Should not match if kind doesn't match.
        let url = resolve_rust_path("enum@MyStruct", &index, "mycrate", 0);
        assert_eq!(url, None);
    }

    #[test]
    fn test_resolve_rust_path_unresolved() {
        let index = test_index();
        let url = resolve_rust_path("NonExistent", &index, "mycrate", 0);
        assert_eq!(url, None);
    }

    #[test]
    fn test_build_url_various_kinds() {
        let struct_loc = ItemLocation {
            crate_name: "foo".to_string(),
            path: vec!["foo".to_string(), "Bar".to_string()],
            kind: ItemKind::Struct,
        };
        assert_eq!(build_url(&struct_loc, 0), "foo/struct.Bar.html");

        let enum_loc = ItemLocation {
            crate_name: "foo".to_string(),
            path: vec!["foo".to_string(), "mod".to_string(), "Baz".to_string()],
            kind: ItemKind::Enum,
        };
        assert_eq!(build_url(&enum_loc, 1), "../foo/mod/enum.Baz.html");

        let fn_loc = ItemLocation {
            crate_name: "foo".to_string(),
            path: vec!["foo".to_string(), "func".to_string()],
            kind: ItemKind::Function,
        };
        assert_eq!(build_url(&fn_loc, 2), "../../foo/fn.func.html");
    }
}
