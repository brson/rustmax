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
    tera_ctx.insert("module_path", &path);

    // Module documentation.
    let docs = tree.module_item.as_ref()
        .and_then(|m| m.item.docs.as_ref())
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Categorize items.
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
                .map(|m| m.html_path.display().to_string())
                .unwrap_or_default(),
            short_doc: submodule.module_item.as_ref()
                .and_then(|m| m.item.docs.as_ref())
                .map(|d| first_sentence(d))
                .unwrap_or_default(),
        });
    }

    for item in &tree.items {
        let summary = ItemSummary {
            name: item.item.name.clone().unwrap_or_default(),
            path: item.html_path.display().to_string(),
            short_doc: item.item.docs.as_ref()
                .map(|d| first_sentence(d))
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

    // Path to root for CSS and links.
    // For modules, the file is at <path>/index.html, so depth equals path length.
    let depth = path.len();
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
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

/// Extract the first sentence from documentation.
fn first_sentence(docs: &str) -> String {
    let trimmed = docs.trim();
    if let Some(pos) = trimmed.find(". ") {
        trimmed[..pos + 1].to_string()
    } else if let Some(pos) = trimmed.find(".\n") {
        trimmed[..pos + 1].to_string()
    } else if trimmed.ends_with('.') {
        trimmed.lines().next().unwrap_or(trimmed).to_string()
    } else {
        trimmed.lines().next().unwrap_or(trimmed).to_string()
    }
}
