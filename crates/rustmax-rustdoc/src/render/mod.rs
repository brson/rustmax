//! Documentation rendering.

pub mod module;
pub mod item;
pub mod signature;
pub mod markdown;
pub mod highlight;
pub mod sidebar;

use rmx::prelude::*;
use rmx::tera::Tera;
use rustdoc_types::{Crate, Id};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{RenderConfig, ModuleTree};
use crate::types::{build_module_tree, build_impl_index, ImplIndex};

/// Context for rendering documentation.
pub struct RenderContext<'a> {
    /// The rustdoc crate data.
    pub krate: &'a Crate,
    /// Rendering configuration.
    pub config: &'a RenderConfig,
    /// Template engine.
    pub tera: Tera,
    /// Map from item ID to its full path.
    pub id_to_path: HashMap<&'a Id, Vec<String>>,
    /// The module tree.
    pub module_tree: ModuleTree<'a>,
    /// Syntax highlighter.
    pub highlighter: highlight::Highlighter,
    /// Index of impl blocks.
    pub impl_index: ImplIndex<'a>,
}

impl<'a> RenderContext<'a> {
    /// Create a new render context.
    pub fn new(krate: &'a Crate, config: &'a RenderConfig) -> AnyResult<Self> {
        let tera = load_templates()?;
        let id_to_path = build_id_to_path(krate);
        let module_tree = build_module_tree(krate, config.include_private)?;
        let highlighter = highlight::Highlighter::new();
        let impl_index = build_impl_index(krate);

        Ok(Self {
            krate,
            config,
            tera,
            id_to_path,
            module_tree,
            highlighter,
            impl_index,
        })
    }

    /// Get the crate name.
    pub fn crate_name(&self) -> &str {
        self.module_tree.name.as_str()
    }

    /// Resolve an item ID to its HTML path relative to output root.
    pub fn resolve_path(&self, id: &Id) -> Option<PathBuf> {
        // First check our computed paths.
        if let Some(path) = self.id_to_path.get(id) {
            return Some(path_to_html_file(path));
        }
        None
    }

    /// Resolve an item ID to a relative URL from a given depth.
    ///
    /// Returns the URL path to the item's documentation page, relative to the current page.
    /// The `current_depth` is how many path segments deep the current page is.
    pub fn resolve_item_url(&self, id: &Id, current_depth: usize) -> Option<String> {
        // Look up the path and kind from krate.paths.
        let summary = self.krate.paths.get(id)?;

        // Only link to items in this crate.
        if summary.crate_id != 0 {
            return None;
        }

        let kind_prefix = match summary.kind {
            rustdoc_types::ItemKind::Struct => "struct.",
            rustdoc_types::ItemKind::Enum => "enum.",
            rustdoc_types::ItemKind::Trait => "trait.",
            rustdoc_types::ItemKind::Function => "fn.",
            rustdoc_types::ItemKind::TypeAlias => "type.",
            rustdoc_types::ItemKind::Constant => "constant.",
            rustdoc_types::ItemKind::Static => "static.",
            rustdoc_types::ItemKind::Macro => "macro.",
            rustdoc_types::ItemKind::Module => "",
            _ => return None, // Don't link to other kinds.
        };

        let path = &summary.path;
        if path.is_empty() {
            return None;
        }

        // Build the path to root from current depth.
        let path_to_root = if current_depth == 0 {
            String::new()
        } else {
            "../".repeat(current_depth)
        };

        // Build the URL.
        let (dir_parts, name) = path.split_at(path.len() - 1);
        let mut url = path_to_root;
        for part in dir_parts {
            url.push_str(part);
            url.push('/');
        }

        if summary.kind == rustdoc_types::ItemKind::Module {
            url.push_str(&name[0]);
            url.push_str("/index.html");
        } else {
            url.push_str(kind_prefix);
            url.push_str(&name[0]);
            url.push_str(".html");
        }

        Some(url)
    }

    /// Render markdown to HTML.
    pub fn render_markdown(&self, md: &str) -> String {
        markdown::render_markdown(md, &self.highlighter)
    }
}

fn load_templates() -> AnyResult<Tera> {
    let mut tera = Tera::default();

    // Register templates from embedded strings.
    tera.add_raw_template("base.html", include_str!("../templates/base.html"))?;
    tera.add_raw_template("module.html", include_str!("../templates/module.html"))?;
    tera.add_raw_template("struct.html", include_str!("../templates/struct.html"))?;
    tera.add_raw_template("enum.html", include_str!("../templates/enum.html"))?;
    tera.add_raw_template("trait.html", include_str!("../templates/trait.html"))?;
    tera.add_raw_template("function.html", include_str!("../templates/function.html"))?;
    tera.add_raw_template("type_alias.html", include_str!("../templates/type_alias.html"))?;
    tera.add_raw_template("constant.html", include_str!("../templates/constant.html"))?;
    tera.add_raw_template("macro.html", include_str!("../templates/macro.html"))?;
    tera.add_raw_template("sidebar.html", include_str!("../templates/sidebar.html"))?;

    Ok(tera)
}

fn build_id_to_path<'a>(krate: &'a Crate) -> HashMap<&'a Id, Vec<String>> {
    let mut map = HashMap::new();
    for (id, summary) in &krate.paths {
        map.insert(id, summary.path.clone());
    }
    map
}

fn path_to_html_file(path: &[String]) -> PathBuf {
    if path.is_empty() {
        return PathBuf::from("index.html");
    }

    let mut result = PathBuf::new();
    for (i, part) in path.iter().enumerate() {
        if i == path.len() - 1 {
            // Last element is the item name.
            result.push(format!("{}.html", part));
        } else {
            result.push(part);
        }
    }
    result
}
