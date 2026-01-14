//! Output file writing.

use rmx::prelude::*;
use rustdoc_types::{ItemEnum, ItemKind};
use std::fs;
use std::path::{Path, PathBuf};

use crate::render::{self, RenderContext};
use crate::types::{ModuleTree, RenderableItem};

/// Write all documentation to the output directory.
pub fn write_docs(ctx: &RenderContext) -> AnyResult<()> {
    // Create output directory.
    fs::create_dir_all(&ctx.config.output_dir)
        .with_context(|| format!("Failed to create output directory: {}", ctx.config.output_dir.display()))?;

    // Write CSS.
    write_css(&ctx.config.output_dir)?;

    // Write module pages recursively.
    write_module_tree(ctx, &ctx.module_tree)?;

    info!("Documentation written to {}", ctx.config.output_dir.display());

    Ok(())
}

fn write_module_tree(ctx: &RenderContext, tree: &ModuleTree) -> AnyResult<()> {
    // Write this module's page.
    if let Some(ref module_item) = tree.module_item {
        let html = render::module::render_module(ctx, tree)?;
        let out_path = ctx.config.output_dir.join(&module_item.html_path);

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&out_path, &html)
            .with_context(|| format!("Failed to write {}", out_path.display()))?;
    }

    // Write item pages.
    for item in &tree.items {
        // For re-exports, look up the target item and render it.
        let (render_item, html_path) = if let ItemEnum::Use(use_item) = &item.item.inner {
            if use_item.is_glob || use_item.id.is_none() {
                continue;
            }
            let target_id = use_item.id.as_ref().unwrap();
            let Some(target_item) = ctx.krate.index.get(target_id) else {
                continue;
            };
            // Create a renderable item for the target at the re-export path.
            let target_renderable = crate::types::RenderableItem {
                id: target_id,
                item: target_item,
                path: item.path.clone(),
                html_path: build_reexport_html_path(&item.path, &target_item.inner),
            };
            (target_renderable, build_reexport_html_path(&item.path, &target_item.inner))
        } else {
            (item.clone(), item.html_path.clone())
        };

        let html = match &render_item.item.inner {
            ItemEnum::Struct(_) => render::item::render_struct(ctx, &render_item)?,
            ItemEnum::Enum(_) => render::item::render_enum(ctx, &render_item)?,
            ItemEnum::Trait(_) => render::item::render_trait(ctx, &render_item)?,
            ItemEnum::Function(_) => render::item::render_function(ctx, &render_item)?,
            ItemEnum::TypeAlias(_) => render::item::render_type_alias(ctx, &render_item)?,
            ItemEnum::Constant { .. } | ItemEnum::Static(_) => render::item::render_constant(ctx, &render_item)?,
            ItemEnum::Macro(_) => render::item::render_macro(ctx, &render_item)?,
            _ => continue,
        };

        let out_path = ctx.config.output_dir.join(&html_path);

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&out_path, &html)
            .with_context(|| format!("Failed to write {}", out_path.display()))?;
    }

    // Recurse into submodules.
    for submodule in &tree.submodules {
        write_module_tree(ctx, submodule)?;
    }

    // Write pages for glob-reexported items.
    if let Some(all_crates) = ctx.all_crates {
        let module_path = tree.module_item.as_ref()
            .map(|m| m.path.clone())
            .unwrap_or_default();

        for glob in &tree.glob_reexports {
            if let Some(target_krate) = all_crates.get(&glob.target_crate) {
                write_glob_reexport_items(ctx, target_krate, &module_path)?;
            }
        }
    }

    Ok(())
}

/// Write pages for items from a glob re-export.
fn write_glob_reexport_items(
    ctx: &RenderContext,
    target_krate: &rustdoc_types::Crate,
    module_path: &[String],
) -> AnyResult<()> {
    // Get the target crate's root module items.
    let Some(root_item) = target_krate.index.get(&target_krate.root) else {
        return Ok(());
    };

    let ItemEnum::Module(module) = &root_item.inner else {
        return Ok(());
    };

    for child_id in &module.items {
        let Some(child_item) = target_krate.index.get(child_id) else {
            continue;
        };

        // Skip private and unnamed items.
        if child_item.visibility != rustdoc_types::Visibility::Public {
            continue;
        }

        let item_name = child_item.name.clone().unwrap_or_default();
        if item_name.is_empty() {
            continue;
        }

        // Skip modules - they need special handling.
        if matches!(child_item.inner, ItemEnum::Module(_)) {
            continue;
        }

        // Build the path for the re-exported item.
        let mut item_path = module_path.to_vec();
        item_path.push(item_name);

        let html_path = build_reexport_html_path(&item_path, &child_item.inner);

        // Create a renderable item.
        let renderable = RenderableItem {
            id: child_id,
            item: child_item,
            path: item_path,
            html_path: html_path.clone(),
        };

        let html = match &child_item.inner {
            ItemEnum::Struct(_) => render::item::render_struct(ctx, &renderable)?,
            ItemEnum::Enum(_) => render::item::render_enum(ctx, &renderable)?,
            ItemEnum::Trait(_) => render::item::render_trait(ctx, &renderable)?,
            ItemEnum::Function(_) => render::item::render_function(ctx, &renderable)?,
            ItemEnum::TypeAlias(_) => render::item::render_type_alias(ctx, &renderable)?,
            ItemEnum::Constant { .. } | ItemEnum::Static(_) => render::item::render_constant(ctx, &renderable)?,
            ItemEnum::Macro(_) => render::item::render_macro(ctx, &renderable)?,
            _ => continue,
        };

        let out_path = ctx.config.output_dir.join(&html_path);

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&out_path, &html)
            .with_context(|| format!("Failed to write {}", out_path.display()))?;
    }

    Ok(())
}

/// Build HTML path for a glob re-exported item.
fn build_reexport_html_path(path: &[String], inner: &ItemEnum) -> PathBuf {
    let kind = match inner {
        ItemEnum::Struct(_) => Some(ItemKind::Struct),
        ItemEnum::Enum(_) => Some(ItemKind::Enum),
        ItemEnum::Trait(_) => Some(ItemKind::Trait),
        ItemEnum::Function(_) => Some(ItemKind::Function),
        ItemEnum::TypeAlias(_) => Some(ItemKind::TypeAlias),
        ItemEnum::Constant { .. } => Some(ItemKind::Constant),
        ItemEnum::Static(_) => Some(ItemKind::Static),
        ItemEnum::Macro(_) => Some(ItemKind::Macro),
        ItemEnum::Union(_) => Some(ItemKind::Union),
        ItemEnum::Module(_) => Some(ItemKind::Module),
        _ => None,
    };

    let prefix = match kind {
        Some(ItemKind::Module) => "",
        Some(ItemKind::Struct) => "struct.",
        Some(ItemKind::Enum) => "enum.",
        Some(ItemKind::Trait) => "trait.",
        Some(ItemKind::Function) => "fn.",
        Some(ItemKind::TypeAlias) => "type.",
        Some(ItemKind::Constant) => "constant.",
        Some(ItemKind::Static) => "static.",
        Some(ItemKind::Macro) => "macro.",
        Some(ItemKind::Union) => "union.",
        _ => "",
    };

    if path.is_empty() {
        return PathBuf::from("index.html");
    }

    let (dir_parts, name) = path.split_at(path.len() - 1);
    let mut result = PathBuf::new();

    for part in dir_parts {
        result.push(part);
    }

    if kind == Some(ItemKind::Module) {
        result.push(&name[0]);
        result.push("index.html");
    } else {
        let filename = format!("{}{}.html", prefix, name[0]);
        result.push(filename);
    }

    result
}

fn write_css(output_dir: &Path) -> AnyResult<()> {
    let css = generate_css();
    let path = output_dir.join("rustdoc.css");
    fs::write(&path, css)
        .with_context(|| format!("Failed to write CSS to {}", path.display()))
}

fn generate_css() -> &'static str {
    r#"/* rustmax-rustdoc styles */
:root {
    --bg: #fff;
    --fg: #333;
    --link: #0066cc;
    --sidebar-bg: #f5f5f5;
    --sidebar-width: 280px;
    --code-bg: #f4f4f4;
    --border: #ddd;
}

@media (prefers-color-scheme: dark) {
    :root {
        --bg: #1a1a1a;
        --fg: #ddd;
        --link: #6db3f2;
        --sidebar-bg: #252525;
        --code-bg: #2a2a2a;
        --border: #444;
    }
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    background: var(--bg);
    color: var(--fg);
}

body {
    display: flex;
    min-height: 100vh;
}

a { color: var(--link); text-decoration: none; }
a:hover { text-decoration: underline; }

/* Nav toggle button */
.nav-toggle {
    position: sticky;
    top: 0;
    z-index: 100;
    width: 36px;
    height: 36px;
    margin: 0.5rem;
    padding: 0;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg);
    cursor: pointer;
    flex-shrink: 0;
}

.nav-toggle::before {
    content: '';
    display: block;
    width: 18px;
    height: 2px;
    margin: 7px auto;
    background: var(--fg);
    box-shadow: 0 5px 0 var(--fg), 0 10px 0 var(--fg);
}

.nav-toggle:hover {
    background: var(--sidebar-bg);
}

/* Sidebar */
.sidebar {
    flex-shrink: 0;
    width: var(--sidebar-width);
    height: 100vh;
    position: sticky;
    top: 0;
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--border);
    padding: 1rem;
    transition: width 0.2s ease, padding 0.2s ease;
}

.nav-collapsed .sidebar {
    width: 0;
    padding: 0;
    border-right: none;
}

.sidebar-title {
    font-weight: bold;
    font-size: 1.2rem;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
    white-space: nowrap;
}

.sidebar-title a { color: var(--fg); }

.module-tree {
    list-style: none;
    padding: 0;
    margin: 0;
}

.module-tree ul {
    list-style: none;
    padding-left: 1rem;
    margin: 0;
}

.module-tree li {
    margin: 0.2rem 0;
    white-space: nowrap;
}

.module-tree .current > a { font-weight: bold; }

/* Main content */
main {
    flex: 1;
    min-width: 0;
    max-width: 50rem;
    margin: 0 auto;
    padding: 2rem 3rem;
}

header { margin-bottom: 2rem; }

.breadcrumbs {
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 0.5rem;
}

h1 {
    font-size: 1.8rem;
    margin: 0;
}

h1 code {
    font-size: inherit;
    background: none;
    padding: 0;
}

h2 {
    font-size: 1.3rem;
    margin-top: 2rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.3rem;
}

/* Code */
code {
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 0.9em;
    background: var(--code-bg);
    padding: 0.1em 0.3em;
    border-radius: 3px;
}

pre {
    background: var(--code-bg);
    padding: 1rem;
    overflow-x: auto;
    border-radius: 4px;
    border: 1px solid var(--border);
}

pre code {
    background: none;
    padding: 0;
}

.signature pre {
    margin: 0;
}

/* Item lists */
.item-list ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.item-list li {
    padding: 0.3rem 0;
    border-bottom: 1px solid var(--border);
}

.item-list li:last-child { border-bottom: none; }

.short-doc {
    display: block;
    font-size: 0.9rem;
    color: #666;
    margin-top: 0.2rem;
}

/* Fields */
.fields dl {
    margin: 0;
}

.fields dt {
    margin-top: 1rem;
    font-weight: bold;
}

.fields dd {
    margin-left: 1rem;
    margin-top: 0.3rem;
}

/* Docs section */
.docs {
    margin-top: 1.5rem;
}

.docs p:first-child { margin-top: 0; }

/* Tables */
table {
    border-collapse: collapse;
    width: 100%;
    margin: 1rem 0;
}

th, td {
    border: 1px solid var(--border);
    padding: 0.5rem;
    text-align: left;
}

th { background: var(--code-bg); }

/* Blockquotes */
blockquote {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 4px solid var(--border);
    background: var(--code-bg);
}

/* Mobile */
@media (max-width: 768px) {
    body { flex-direction: column; }

    .nav-toggle {
        position: relative;
        width: 100%;
        margin: 0;
        border-radius: 0;
        border-left: none;
        border-right: none;
        border-top: none;
    }

    .sidebar {
        position: relative;
        width: 100%;
        height: auto;
        max-height: 60vh;
        border-right: none;
        border-bottom: 1px solid var(--border);
        transition: max-height 0.2s ease, padding 0.2s ease;
    }

    .nav-collapsed .sidebar {
        width: 100%;
        max-height: 0;
        overflow: hidden;
        border-bottom: none;
    }

    .module-tree li {
        white-space: normal;
    }

    main { padding: 1rem; }
}
"#
}
