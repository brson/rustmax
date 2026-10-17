//! Sidebar navigation.
//!
//! The sidebar is the crate's whole module tree, the same on every page but
//! for the `../` prefix of its links and the module marked as current.
//! Inlined into every page, it made a crate's output grow with its pages
//! times its modules, so the tree is instead written once per crate, by
//! [`tree_script`], and built into each page by `rustdoc-sidebar.js`.

use serde_json::{Value, json};
use std::path::PathBuf;

use crate::types::ModuleTree;

/// The script that builds a page's sidebar from its crate's tree.
pub const SIDEBAR_JS: &str = include_str!("../../assets/rustdoc-sidebar.js");

/// The crate's module tree as a script defining `rmxSidebarTree`.
///
/// Each module is a `[name, href, path, submodules]` array,
/// with `href` relative to the output directory and `path` joined by `::`.
pub fn tree_script(tree: &ModuleTree) -> String {
    format!("window.rmxSidebarTree = {};\n", tree_node(tree))
}

/// Where [`tree_script`] is written, relative to the output directory.
pub fn tree_script_path(tree: &ModuleTree) -> PathBuf {
    let root = tree.module_item.as_ref().expect("the crate root has a module item");
    root.html_path.with_file_name("sidebar-tree.js")
}

fn tree_node(tree: &ModuleTree) -> Value {
    let (href, path) = match &tree.module_item {
        Some(m) => (m.html_path.display().to_string(), m.path.join("::")),
        None => ("index.html".to_string(), String::new()),
    };
    let submodules: Vec<Value> = tree.submodules.iter().map(tree_node).collect();
    json!([tree.name, href, path, submodules])
}
