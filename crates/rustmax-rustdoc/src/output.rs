//! Output file writing.

use rmx::prelude::*;
use rustdoc_types::ItemEnum;
use std::fs;
use std::path::Path;

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

    // Write item pages.
    for item in &tree.items {
        let html = match &item.item.inner {
            ItemEnum::Struct(_) => render::item::render_struct(ctx, item)?,
            ItemEnum::Enum(_) => render::item::render_enum(ctx, item)?,
            ItemEnum::Trait(_) => render::item::render_trait(ctx, item)?,
            ItemEnum::Function(_) => render::item::render_function(ctx, item)?,
            ItemEnum::TypeAlias(_) => render::item::render_type_alias(ctx, item)?,
            ItemEnum::Constant { .. } | ItemEnum::Static(_) => render::item::render_constant(ctx, item)?,
            ItemEnum::Macro(_) => render::item::render_macro(ctx, item)?,
            _ => continue,
        };

        let out_path = ctx.config.output_dir.join(&item.html_path);

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

    Ok(())
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
    --sidebar-width: 260px;
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

/* Sidebar */
.sidebar {
    flex-shrink: 0;
    width: var(--sidebar-width);
    height: 100vh;
    position: sticky;
    top: 0;
    overflow-y: auto;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--border);
    padding: 1rem;
}

.sidebar-title {
    font-weight: bold;
    font-size: 1.2rem;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
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

.module-tree li { margin: 0.2rem 0; }
.module-tree .current > a { font-weight: bold; }

/* Main content */
main {
    flex: 1;
    min-width: 0;
    max-width: 60rem;
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

    .sidebar {
        position: relative;
        width: 100%;
        height: auto;
        max-height: 50vh;
        border-right: none;
        border-bottom: 1px solid var(--border);
    }

    main { padding: 1rem; }
}
"#
}
