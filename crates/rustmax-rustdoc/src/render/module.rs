//! Module page rendering.

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::ItemEnum;

use super::{RenderContext, ReexportTarget};
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
    // For modules, depth = path.len() (file is at <path>/index.html).
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&path, path.len()));

    // Determine if this is the crate root (path has just the crate name).
    let is_crate_root = path.len() == 1;
    tera_ctx.insert("is_crate_root", &is_crate_root);

    // Path to root for CSS and links.
    // For modules, the file is at <path>/index.html, so depth equals path length.
    let depth = path.len();
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Pre-resolve links from the module item's links field.
    let item_links = tree.module_item.as_ref()
        .map(|m| ctx.resolve_item_links(&m.item.links, depth))
        .unwrap_or_default();

    // Module documentation.
    let docs = tree.module_item.as_ref()
        .and_then(|m| m.item.docs.as_ref())
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &item_links))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Categorize items.
    // Item paths are absolute from output root, so prepend path_to_root to make relative.
    let mut modules = Vec::new();
    let mut structs = Vec::new();
    let mut unions = Vec::new();
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
                .map(|d| ctx.render_short_doc(d, depth))
                .unwrap_or_default(),
        });
    }

    for item in &tree.items {
        // Handle re-exports specially.
        if let ItemEnum::Use(use_item) = &item.item.inner {
            if use_item.is_glob || use_item.id.is_none() {
                continue;
            }
            let target_id = use_item.id.as_ref().unwrap();

            // Determine link URL.
            let item_url = match ctx.reexport_target(target_id) {
                ReexportTarget::External { ref path, kind } => {
                    // Link to external crate's page.
                    ctx.build_item_url(path, kind, depth).unwrap_or_default()
                }
                ReexportTarget::LocalPublic { .. } | ReexportTarget::NeedsPage => {
                    // Link to re-export's page at re-export location.
                    format!("{}{}", path_to_root, item.html_path.display())
                }
            };

            // Try to get target from index first, then fall back to paths.
            if let Some(target_item) = ctx.krate.index.get(target_id) {
                let target_links = ctx.resolve_item_links(&target_item.links, depth);
                let summary = ItemSummary {
                    name: use_item.name.clone(),
                    path: item_url,
                    short_doc: target_item.docs.as_ref()
                        .map(|d| ctx.render_short_doc_with_item_links(d, depth, &target_links))
                        .unwrap_or_default(),
                };

                // Categorize by target type.
                match &target_item.inner {
                    ItemEnum::Struct(_) => structs.push(summary),
                    ItemEnum::Union(_) => unions.push(summary),
                    ItemEnum::Enum(_) => enums.push(summary),
                    ItemEnum::Trait(_) => traits.push(summary),
                    ItemEnum::Function(_) => functions.push(summary),
                    ItemEnum::TypeAlias(_) => types.push(summary),
                    ItemEnum::Constant { .. } | ItemEnum::Static(_) => constants.push(summary),
                    ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => macros.push(summary),
                    _ => {}
                }
            } else if let Some(path_info) = ctx.krate.paths.get(target_id) {
                // External re-export - use paths info to categorize.
                let summary = ItemSummary {
                    name: use_item.name.clone(),
                    path: item_url,
                    short_doc: String::new(), // No docs available for external items.
                };

                use rustdoc_types::ItemKind;
                match path_info.kind {
                    ItemKind::Struct => structs.push(summary),
                    ItemKind::Union => unions.push(summary),
                    ItemKind::Enum => enums.push(summary),
                    ItemKind::Trait => traits.push(summary),
                    ItemKind::Function => functions.push(summary),
                    ItemKind::TypeAlias => types.push(summary),
                    ItemKind::Constant | ItemKind::Static => constants.push(summary),
                    ItemKind::Macro => macros.push(summary),
                    ItemKind::Module => modules.push(summary),
                    _ => {}
                }
            }
            continue;
        }

        // Non-Use items.
        let summary = ItemSummary {
            name: item.item.name.clone().unwrap_or_default(),
            path: format!("{}{}", path_to_root, item.html_path.display()),
            short_doc: item.item.docs.as_ref()
                .map(|d| ctx.render_short_doc(d, depth))
                .unwrap_or_default(),
        };

        match &item.item.inner {
            ItemEnum::Struct(_) => structs.push(summary),
            ItemEnum::Union(_) => unions.push(summary),
            ItemEnum::Enum(_) => enums.push(summary),
            ItemEnum::Trait(_) => traits.push(summary),
            ItemEnum::Function(_) => functions.push(summary),
            ItemEnum::TypeAlias(_) => types.push(summary),
            ItemEnum::Constant { .. } | ItemEnum::Static(_) => constants.push(summary),
            ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => macros.push(summary),
            ItemEnum::ExternCrate { name, rename } => {
                // Extern crate appears as a module linking to the external crate.
                let display_name = rename.as_ref().unwrap_or(name);
                let extern_path = format!("{}{}/index.html", path_to_root, display_name);
                modules.push(ItemSummary {
                    name: display_name.clone(),
                    path: extern_path,
                    short_doc: item.item.docs.as_ref()
                        .map(|d| ctx.render_short_doc(d, depth))
                        .unwrap_or_default(),
                });
            }
            _ => {}
        }
    }

    // Add items from glob re-exports, linking to original crate location.
    if let Some(all_crates) = ctx.all_crates {
        for glob in &tree.glob_reexports {
            if let Some(target_krate) = all_crates.get(&glob.target_crate) {
                // Get the target crate's root module items.
                if let Some(root_item) = target_krate.index.get(&target_krate.root) {
                    if let ItemEnum::Module(module) = &root_item.inner {
                        for child_id in &module.items {
                            if let Some(child_item) = target_krate.index.get(child_id) {
                                // Skip private and unnamed items.
                                if child_item.visibility != rustdoc_types::Visibility::Public {
                                    continue;
                                }

                                let item_name = child_item.name.clone().unwrap_or_default();
                                if item_name.is_empty() {
                                    continue;
                                }

                                // Link to the original crate's item page.
                                let item_path = format!(
                                    "{}{}",
                                    path_to_root,
                                    build_original_crate_path(&glob.target_crate, &item_name, &child_item.inner)
                                );

                                let summary = ItemSummary {
                                    name: item_name,
                                    path: item_path,
                                    short_doc: child_item.docs.as_ref()
                                        .map(|d| ctx.render_short_doc(d, depth))
                                        .unwrap_or_default(),
                                };

                                match &child_item.inner {
                                    ItemEnum::Module(_) => modules.push(summary),
                                    ItemEnum::Struct(_) => structs.push(summary),
                                    ItemEnum::Union(_) => unions.push(summary),
                                    ItemEnum::Enum(_) => enums.push(summary),
                                    ItemEnum::Trait(_) => traits.push(summary),
                                    ItemEnum::Function(_) => functions.push(summary),
                                    ItemEnum::TypeAlias(_) => types.push(summary),
                                    ItemEnum::Constant { .. } | ItemEnum::Static(_) => constants.push(summary),
                                    ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => macros.push(summary),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort all item lists alphabetically.
    modules.sort_by(|a, b| a.name.cmp(&b.name));
    structs.sort_by(|a, b| a.name.cmp(&b.name));
    unions.sort_by(|a, b| a.name.cmp(&b.name));
    enums.sort_by(|a, b| a.name.cmp(&b.name));
    traits.sort_by(|a, b| a.name.cmp(&b.name));
    functions.sort_by(|a, b| a.name.cmp(&b.name));
    types.sort_by(|a, b| a.name.cmp(&b.name));
    constants.sort_by(|a, b| a.name.cmp(&b.name));
    macros.sort_by(|a, b| a.name.cmp(&b.name));

    tera_ctx.insert("modules", &modules);
    tera_ctx.insert("structs", &structs);
    tera_ctx.insert("unions", &unions);
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

/// Build HTML path to an item in its original crate.
fn build_original_crate_path(crate_name: &str, item_name: &str, inner: &ItemEnum) -> String {
    let prefix = match inner {
        ItemEnum::Module(_) => "",
        ItemEnum::Struct(_) => "struct.",
        ItemEnum::Enum(_) => "enum.",
        ItemEnum::Trait(_) => "trait.",
        ItemEnum::Function(_) => "fn.",
        ItemEnum::TypeAlias(_) => "type.",
        ItemEnum::Constant { .. } => "constant.",
        ItemEnum::Static(_) => "static.",
        ItemEnum::Macro(_) => "macro.",
        ItemEnum::Union(_) => "union.",
        _ => "",
    };

    // Build path: crate_name/prefix.name.html
    if matches!(inner, ItemEnum::Module(_)) {
        format!("{}/{}/index.html", crate_name, item_name)
    } else {
        format!("{}/{}{}.html", crate_name, prefix, item_name)
    }
}
