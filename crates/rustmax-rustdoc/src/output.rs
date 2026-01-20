//! Output file writing.

use rayon::prelude::*;
use rmx::prelude::*;
use rustdoc_types::{ItemEnum, ItemKind};
use std::fs;
use std::path::{Path, PathBuf};

use crate::render::{self, RenderContext};
use crate::types::ModuleTree;

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

    // Write item pages in parallel.
    tree.items.par_iter().try_for_each(|item| {
        write_item(ctx, item)
    })?;

    // Recurse into submodules in parallel.
    tree.submodules.par_iter().try_for_each(|submodule| {
        write_module_tree(ctx, submodule)
    })
}

/// Write a single item page.
fn write_item(ctx: &RenderContext, item: &crate::types::RenderableItem) -> AnyResult<()> {
    // For re-exports, look up the target item and render it.
    let (render_item, html_path) = if let ItemEnum::Use(use_item) = &item.item.inner {
        if use_item.is_glob || use_item.id.is_none() {
            return Ok(());
        }
        let target_id = use_item.id.as_ref().unwrap();
        let Some(target_item) = ctx.krate.index.get(target_id) else {
            return Ok(());
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
        ItemEnum::Union(_) => render::item::render_union(ctx, &render_item)?,
        ItemEnum::Enum(_) => render::item::render_enum(ctx, &render_item)?,
        ItemEnum::Trait(_) => render::item::render_trait(ctx, &render_item)?,
        ItemEnum::Function(_) => render::item::render_function(ctx, &render_item)?,
        ItemEnum::TypeAlias(_) => render::item::render_type_alias(ctx, &render_item)?,
        ItemEnum::Constant { .. } | ItemEnum::Static(_) => render::item::render_constant(ctx, &render_item)?,
        ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => render::item::render_macro(ctx, &render_item)?,
        _ => return Ok(()),
    };

    let out_path = ctx.config.output_dir.join(&html_path);

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&out_path, &html)
        .with_context(|| format!("Failed to write {}", out_path.display()))
}

/// Build HTML path for a re-exported item.
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
        _ => None,
    };

    let prefix = match kind {
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

    let filename = format!("{}{}.html", prefix, name[0]);
    result.push(filename);

    result
}

fn write_css(output_dir: &Path) -> AnyResult<()> {
    // Copy shared theme CSS (colors and fonts).
    let themes_css = include_str!("../assets/rustmax-themes.css");
    let themes_path = output_dir.join("rustmax-themes.css");
    fs::write(&themes_path, themes_css)
        .with_context(|| format!("Failed to write themes CSS to {}", themes_path.display()))?;

    // Write main rustdoc CSS.
    let css = generate_css();
    let path = output_dir.join("rustdoc.css");
    fs::write(&path, css)
        .with_context(|| format!("Failed to write CSS to {}", path.display()))?;

    // Copy shared syntax highlighting CSS.
    let syntax_css = include_str!("../assets/rustmax-syntax.css");
    let syntax_path = output_dir.join("rustmax-syntax.css");
    fs::write(&syntax_path, syntax_css)
        .with_context(|| format!("Failed to write syntax CSS to {}", syntax_path.display()))
}

fn generate_css() -> &'static str {
    r#"/* rustmax-rustdoc styles
 * Colors and fonts come from rustmax-themes.css
 */
:root {
    --rmx-sidebar-width: 280px;
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    padding: 0;
    font: var(--rmx-font-text);
    line-height: 1.6;
    background: var(--rmx-color-bg);
    color: var(--rmx-color-fg);
}

body {
    display: flex;
    min-height: 100vh;
}

a { color: var(--rmx-color-links); text-decoration: none; }
a:hover { text-decoration: underline; }

/* Nav toggle button */
.nav-toggle {
    position: sticky;
    top: 0.5rem;
    align-self: flex-start;
    flex-shrink: 0;
    width: 36px;
    height: 36px;
    margin: 0.5rem;
    padding: 0;
    border: 1px solid var(--rmx-color-border);
    border-radius: 4px;
    background: var(--rmx-color-bg);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
}

.nav-toggle::before {
    content: '';
    width: 18px;
    height: 2px;
    background: var(--rmx-color-fg);
    box-shadow: 0 6px 0 var(--rmx-color-fg), 0 -6px 0 var(--rmx-color-fg);
}

.nav-toggle:hover {
    background: var(--rmx-color-bg-alt);
}

/* Sidebar */
.sidebar {
    flex-shrink: 0;
    width: var(--rmx-sidebar-width);
    height: 100vh;
    position: sticky;
    top: 0;
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--rmx-color-bg-alt);
    border-right: 1px solid var(--rmx-color-border);
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
    border-bottom: 1px solid var(--rmx-color-border);
    white-space: nowrap;
}

.sidebar-title a { color: var(--rmx-color-fg); }

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
    border-bottom: 1px solid var(--rmx-color-border);
    padding-bottom: 0.3rem;
}

/* Code */
code {
    font: var(--rmx-font-code);
    font-size: 0.9em;
    background: var(--rmx-color-bg-alt);
    padding: 0.1em 0.3em;
    border-radius: 3px;
}

pre {
    background: var(--rmx-color-bg-alt);
    padding: 1rem;
    overflow-x: auto;
    border-radius: 4px;
    border: 1px solid var(--rmx-color-border);
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
    border-bottom: 1px solid var(--rmx-color-border);
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
    border: 1px solid var(--rmx-color-border);
    padding: 0.5rem;
    text-align: left;
}

th { background: var(--rmx-color-bg-alt); }

/* Blockquotes */
blockquote {
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 4px solid var(--rmx-color-border);
    background: var(--rmx-color-bg-alt);
}

/* Syntax highlighting is in rustmax-syntax.css (shared with www and rmxbook) */

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
        border-bottom: 1px solid var(--rmx-color-border);
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
