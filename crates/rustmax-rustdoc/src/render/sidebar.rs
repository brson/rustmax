//! Sidebar navigation rendering.

use rmx::prelude::*;
use rmx::tera::Context;

use super::RenderContext;
use crate::types::ModuleTree;

/// Render the sidebar HTML for a given current path.
pub fn render_sidebar(ctx: &RenderContext, current_path: &[String]) -> AnyResult<String> {
    let mut tera_ctx = Context::new();

    let tree_html = render_tree(&ctx.module_tree, current_path, 0);
    tera_ctx.insert("tree", &tree_html);
    tera_ctx.insert("crate_name", ctx.crate_name());

    ctx.tera.render("sidebar.html", &tera_ctx)
        .context("Failed to render sidebar template")
}

fn render_tree(tree: &ModuleTree, current_path: &[String], depth: usize) -> String {
    let mut html = String::new();

    let is_current = tree.module_item.as_ref()
        .map(|m| m.path == current_path)
        .unwrap_or(false);

    let class = if is_current { " class=\"current\"" } else { "" };

    let href = tree.module_item.as_ref()
        .map(|m| m.html_path.display().to_string())
        .unwrap_or_else(|| "index.html".to_string());

    // Calculate relative path from root.
    let rel_href = if depth == 0 {
        href
    } else {
        format!("{}{}", "../".repeat(depth), href)
    };

    html.push_str(&format!(
        "<li{}><a href=\"{}\">{}</a>",
        class,
        html_escape(&rel_href),
        html_escape(&tree.name)
    ));

    if !tree.submodules.is_empty() {
        html.push_str("<ul>");
        for submodule in &tree.submodules {
            html.push_str(&render_tree(submodule, current_path, depth));
        }
        html.push_str("</ul>");
    }

    html.push_str("</li>\n");

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
