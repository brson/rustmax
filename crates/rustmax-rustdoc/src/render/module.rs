//! Module page rendering.

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::ItemEnum;

use super::RenderContext;
use crate::types::ModuleTree;

/// Render a module page to HTML.
pub fn render_module(ctx: &RenderContext, tree: &ModuleTree) -> AnyResult<String> {
    let mut tera_ctx = Context::new();

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("module_name", &tree.name);

    // Build module path for breadcrumbs.
    let path = tree.module_item.as_ref()
        .map(|m| m.path.clone())
        .unwrap_or_default();

    // Build breadcrumbs with URLs.
    let breadcrumbs: Vec<Breadcrumb> = path.iter().enumerate().map(|(i, name)| {
        let url = if i == path.len() - 1 {
            // Current page, no link.
            None
        } else {
            // Link to ancestor module: go up (path.len() - 1 - i) levels.
            let ups = path.len() - 1 - i;
            Some(format!("{}index.html", "../".repeat(ups)))
        };
        Breadcrumb { name: name.clone(), url }
    }).collect();
    tera_ctx.insert("breadcrumbs", &breadcrumbs);

    // Path to root for CSS and links.
    // For modules, the file is at <path>/index.html, so depth equals path length.
    let depth = path.len();
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Module documentation.
    let docs = tree.module_item.as_ref()
        .and_then(|m| m.item.docs.as_ref())
        .map(|d| ctx.render_markdown_with_links(d, depth))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Categorize items.
    // Item paths are absolute from output root, so prepend path_to_root to make relative.
    let mut modules = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut traits = Vec::new();
    let mut functions = Vec::new();
    let mut types = Vec::new();
    let mut constants = Vec::new();
    let mut macros = Vec::new();

    for submodule in &tree.submodules {
        modules.push(ItemSummary {
            name: submodule.name.clone(),
            path: submodule.module_item.as_ref()
                .map(|m| format!("{}{}", path_to_root, m.html_path.display()))
                .unwrap_or_default(),
            short_doc: submodule.module_item.as_ref()
                .and_then(|m| m.item.docs.as_ref())
                .map(|d| first_paragraph(d))
                .unwrap_or_default(),
        });
    }

    for item in &tree.items {
        let summary = ItemSummary {
            name: item.item.name.clone().unwrap_or_default(),
            path: format!("{}{}", path_to_root, item.html_path.display()),
            short_doc: item.item.docs.as_ref()
                .map(|d| first_paragraph(d))
                .unwrap_or_default(),
        };

        match &item.item.inner {
            ItemEnum::Struct(_) => structs.push(summary),
            ItemEnum::Enum(_) => enums.push(summary),
            ItemEnum::Trait(_) => traits.push(summary),
            ItemEnum::Function(_) => functions.push(summary),
            ItemEnum::TypeAlias(_) => types.push(summary),
            ItemEnum::Constant { .. } | ItemEnum::Static(_) => constants.push(summary),
            ItemEnum::Macro(_) => macros.push(summary),
            _ => {}
        }
    }

    tera_ctx.insert("modules", &modules);
    tera_ctx.insert("structs", &structs);
    tera_ctx.insert("enums", &enums);
    tera_ctx.insert("traits", &traits);
    tera_ctx.insert("functions", &functions);
    tera_ctx.insert("types", &types);
    tera_ctx.insert("constants", &constants);
    tera_ctx.insert("macros", &macros);
    tera_ctx.insert("path_to_root", &path_to_root);

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("module.html", &tera_ctx)
        .context("Failed to render module template")
}

#[derive(serde::Serialize)]
struct ItemSummary {
    name: String,
    path: String,
    short_doc: String,
}

#[derive(serde::Serialize)]
struct Breadcrumb {
    name: String,
    url: Option<String>,
}

/// Extract the first paragraph from documentation.
///
/// Strips reference-style link definitions that might appear in the text.
fn first_paragraph(docs: &str) -> String {
    let trimmed = docs.trim();

    // Find the first blank line (paragraph break).
    let first_para = if let Some(pos) = trimmed.find("\n\n") {
        &trimmed[..pos]
    } else {
        trimmed
    };

    // Filter out lines that look like reference definitions: [label]: url
    // These start with [ and contain ]: somewhere.
    let filtered: Vec<&str> = first_para.lines()
        .map(|l| l.trim())
        .filter(|l| {
            // Skip reference definitions.
            if l.starts_with('[') {
                if let Some(bracket_end) = l.find("]:") {
                    // Check the part before ]: is a valid label (no nested brackets).
                    let label = &l[1..bracket_end];
                    if !label.contains('[') && !label.contains(']') {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    filtered.join(" ")
}
