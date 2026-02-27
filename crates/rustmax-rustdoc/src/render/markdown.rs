//! Markdown to HTML rendering with syntax highlighting and intra-doc link resolution.

use std::collections::HashMap;

use comrak::{markdown_to_html_with_plugins, Arena, Options, Plugins};
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::NodeValue;
use rustdoc_types::ItemKind;

use super::highlight::Highlighter;
use crate::{GlobalItemIndex, ItemLocation};

/// Render markdown to HTML with syntax highlighting.
pub fn render_markdown(md: &str, highlighter: &Highlighter) -> String {
    let options = markdown_options();

    let adapter = HighlightAdapter { highlighter };
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(md, &options, &plugins)
}

/// Render markdown to HTML with intra-doc link resolution.
///
/// Resolves Rust path references like `` [`Vec`] `` or `[text](std::option::Option)`
/// to actual HTML documentation URLs.
///
/// `pre_resolved_links` maps link text to pre-resolved URLs from the item's
/// rustdoc JSON `links` field. These take priority over global index resolution.
pub fn render_markdown_with_links(
    md: &str,
    highlighter: &Highlighter,
    global_index: Option<&GlobalItemIndex>,
    current_crate: &str,
    current_depth: usize,
    pre_resolved_links: &HashMap<String, String>,
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
            // Check pre-resolved links first (from rustdoc's links field).
            if let Some(url) = pre_resolved_links.get(&link.url) {
                link.url = url.clone();
            } else if is_rust_path(&link.url) {
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
    options.extension.header_ids = Some("".to_string());
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

    // Match [`path::to::Item`] or [`::crate::Item`] - we'll check for trailing ( or [ manually.
    let re = Regex::new(
        r#"\[`((?:::)?[a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)`\]"#
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

    // Track if path explicitly refers to external crate (starts with ::).
    let is_external_crate_ref = clean_path.starts_with("::");

    // Handle crate:: prefix and :: prefix for external crates.
    let lookup_path = if let Some(rest) = clean_path.strip_prefix("crate::") {
        format!("{}::{}", current_crate, rest)
    } else if let Some(rest) = clean_path.strip_prefix("::") {
        // `::crate_name::path` refers to an external crate, not current crate.
        rest.to_string()
    } else {
        clean_path.to_string()
    };

    // std re-exports most of core/alloc, so try core::/alloc:: FIRST.
    // This ensures std::error::Error resolves to core, not a crate's re-export.
    // Also handle crate::std::... patterns (e.g., rustmax::std::error::Error).
    let std_rest = lookup_path.strip_prefix("std::")
        .or_else(|| lookup_path.strip_prefix(&format!("{}::std::", current_crate)));
    if let Some(rest) = std_rest {
        for alt_crate in ["core", "alloc"] {
            let alt_lookup = format!("{}::{}", alt_crate, rest);
            if let Some(location) = index.items.get(&alt_lookup) {
                if matches_kind_filter(location, kind_filter) {
                    return Some(build_url(location, current_depth));
                }
            }
        }
    }

    // For paths without ::, try current crate prefix FIRST.
    // This ensures `anyhow` in rustmax docs resolves to `rustmax::anyhow` (module)
    // rather than `anyhow` (external crate).
    // BUT skip this if the path explicitly started with :: (external crate reference).
    if !is_external_crate_ref && !lookup_path.contains("::") {
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

    // Handle enum variants: link to parent enum with #variant.Name anchor.
    if location.kind == ItemKind::Variant && location.path.len() >= 2 {
        let variant_name = location.path.last().unwrap();
        let enum_path = &location.path[..location.path.len() - 1];
        let enum_name = enum_path.last().unwrap();
        let module_path = &enum_path[..enum_path.len() - 1];
        return format!(
            "{}{}enum.{}.html#variant.{}",
            prefix,
            if module_path.is_empty() {
                String::new()
            } else {
                format!("{}/", module_path.join("/"))
            },
            enum_name,
            variant_name,
        );
    }

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
        ItemKind::ProcDerive => "derive.",
        ItemKind::ProcAttribute => "attr.",
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
        // Extract base language, stripping modifiers like ",ignore", ",no_run", etc.
        // Rustdoc assumes unlabeled code blocks are Rust.
        // "text" means plain text (no highlighting).
        let base_lang = lang
            .map(|l| l.split(',').next().unwrap_or(l).trim())
            .filter(|l| !l.is_empty())
            .unwrap_or("rust");

        // "text" means no syntax highlighting.
        if base_lang == "text" {
            write!(output, "{}", html_escape(code))
        } else {
            let highlighted = self.highlighter.highlight(code, base_lang);
            write!(output, "{}", highlighted)
        }
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

/// Extract first paragraph from markdown.
fn extract_first_paragraph(md: &str) -> String {
    let trimmed = md.trim();
    if let Some(pos) = trimmed.find("\n\n") {
        trimmed[..pos].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Strip wrapping `<p>...</p>` tags from HTML for inline display.
fn strip_paragraph_wrapper(html: &str) -> String {
    let trimmed = html.trim();
    if trimmed.starts_with("<p>") && trimmed.ends_with("</p>") {
        trimmed[3..trimmed.len()-4].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Render the first paragraph of documentation as inline HTML.
///
/// Only the first paragraph is rendered. Rustdoc-style shortcut links
/// within it are resolved via the global index and pre-resolved links,
/// so appending reference definitions from the full doc is unnecessary
/// (and harmful, since many rustdoc ref-def lines are not valid
/// CommonMark and get rendered as visible text).
pub fn render_short_doc(
    full_docs: &str,
    highlighter: &Highlighter,
    global_index: Option<&GlobalItemIndex>,
    current_crate: &str,
    current_depth: usize,
    pre_resolved_links: &HashMap<String, String>,
) -> String {
    let first_para = extract_first_paragraph(full_docs);

    let html = render_markdown_with_links(
        &first_para,
        highlighter,
        global_index,
        current_crate,
        current_depth,
        pre_resolved_links,
    );

    strip_paragraph_wrapper(&html)
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
    fn test_heading_ids() {
        let highlighter = Highlighter::new();
        let html = render_markdown("### Profile: `rmx-profile-no-std`", &highlighter);
        assert!(html.contains("id=\"profile-rmx-profile-no-std\""), "got: {html}");

        let html = render_markdown("### Feature: `rmx-feature-derive`", &highlighter);
        assert!(html.contains("id=\"feature-rmx-feature-derive\""), "got: {html}");

        let html = render_markdown("### Rustlib: `rmx-rustlib-core`", &highlighter);
        assert!(html.contains("id=\"rustlib-rmx-rustlib-core\""), "got: {html}");
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

    #[test]
    fn test_build_url_variant() {
        // Enum variants should produce anchor URLs, not file paths.
        let variant_loc = ItemLocation {
            crate_name: "core".to_string(),
            path: vec!["core".to_string(), "result".to_string(), "Result".to_string(), "Err".to_string()],
            kind: ItemKind::Variant,
        };
        assert_eq!(
            build_url(&variant_loc, 2),
            "../../core/result/enum.Result.html#variant.Err"
        );

        // Variant at crate root.
        let variant_loc = ItemLocation {
            crate_name: "foo".to_string(),
            path: vec!["foo".to_string(), "MyEnum".to_string(), "A".to_string()],
            kind: ItemKind::Variant,
        };
        assert_eq!(
            build_url(&variant_loc, 0),
            "foo/enum.MyEnum.html#variant.A"
        );
    }
}
