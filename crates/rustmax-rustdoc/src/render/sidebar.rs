//! Sidebar navigation rendering.
//!
//! The sidebar is the same module tree on every page of a crate. It varies
//! only by the `../` prefix needed to reach the root and by which module is
//! marked as current, so every distinct sidebar is rendered once up front
//! rather than once per page.

use rmx::prelude::*;
use rmx::tera::{Context, Tera};
use std::collections::HashMap;

use super::signature::html_escape;
use crate::types::ModuleTree;

/// Every sidebar a crate's pages can need.
pub struct Sidebars {
    /// Indexed by page depth, for pages that are not a module's own page.
    by_depth: Vec<String>,
    /// Keyed by module path, holding that module page's depth and sidebar.
    ///
    /// The depth is checked on lookup because a module and an item can share a
    /// path: a `mod copy` and a `fn copy` beside it both have the path
    /// `["alloc", "io", "copy"]`, but their pages sit at different depths.
    by_module: HashMap<Vec<String>, (usize, String)>,
}

impl Sidebars {
    /// Render every sidebar variant for `tree`.
    pub fn build(tree: &ModuleTree, tera: &Tera) -> AnyResult<Self> {
        let max_depth = max_module_depth(tree);

        let mut by_depth = Vec::with_capacity(max_depth + 1);
        for depth in 0..=max_depth {
            // No module is current on an item page.
            by_depth.push(render(tree, &[], &"../".repeat(depth), tera)?);
        }

        let mut module_paths = Vec::new();
        collect_module_paths(tree, &mut module_paths);

        let mut by_module = HashMap::with_capacity(module_paths.len());
        for path in module_paths {
            // A module's page lives at `<path>/index.html`, one level deeper
            // than the path itself.
            let depth = path.len();
            let html = render(tree, &path, &"../".repeat(depth), tera)?;
            by_module.insert(path, (depth, html));
        }

        Ok(Self { by_depth, by_module })
    }

    /// The sidebar for a page at `current_path`, `depth` levels below the root.
    pub fn get(&self, current_path: &[String], depth: usize) -> String {
        if let Some((module_depth, html)) = self.by_module.get(current_path)
            && *module_depth == depth
        {
            return html.clone();
        }
        self.by_depth.get(depth).cloned().unwrap_or_default()
    }
}

fn render(
    tree: &ModuleTree,
    current_path: &[String],
    path_to_root: &str,
    tera: &Tera,
) -> AnyResult<String> {
    let mut tera_ctx = Context::new();
    tera_ctx.insert("tree", &render_tree(tree, current_path, path_to_root));
    tera.render("sidebar.html", &tera_ctx)
        .context("Failed to render sidebar template")
}

fn max_module_depth(tree: &ModuleTree) -> usize {
    let own = tree.module_item.as_ref().map(|m| m.path.len()).unwrap_or(0);
    tree.submodules.iter()
        .map(max_module_depth)
        .max()
        .unwrap_or(0)
        .max(own)
}

fn collect_module_paths(tree: &ModuleTree, paths: &mut Vec<Vec<String>>) {
    if let Some(module_item) = &tree.module_item {
        paths.push(module_item.path.clone());
    }
    for submodule in &tree.submodules {
        collect_module_paths(submodule, paths);
    }
}

fn render_tree(tree: &ModuleTree, current_path: &[String], path_to_root: &str) -> String {
    let mut html = String::new();
    push_tree(tree, current_path, path_to_root, &mut html);
    html
}

fn push_tree(tree: &ModuleTree, current_path: &[String], path_to_root: &str, html: &mut String) {
    let is_current = tree.module_item.as_ref()
        .map(|m| m.path == current_path)
        .unwrap_or(false);

    let class = if is_current { " class=\"current\"" } else { "" };

    let href = tree.module_item.as_ref()
        .map(|m| m.html_path.display().to_string())
        .unwrap_or_else(|| "index.html".to_string());

    // Prepend path_to_root to make the link relative to current page.
    let rel_href = format!("{}{}", path_to_root, href);

    html.push_str(&format!(
        "<li{}><a href=\"{}\">{}</a>",
        class,
        html_escape(&rel_href),
        html_escape(&tree.name)
    ));

    if !tree.submodules.is_empty() {
        html.push_str("<ul>");
        for submodule in &tree.submodules {
            push_tree(submodule, current_path, path_to_root, html);
        }
        html.push_str("</ul>");
    }

    html.push_str("</li>\n");
}
