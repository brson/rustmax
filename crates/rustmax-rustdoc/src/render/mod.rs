//! Documentation rendering.

pub mod module;
pub mod item;
pub mod signature;
pub mod markdown;
pub mod highlight;
pub mod sidebar;

use rmx::prelude::*;
use rmx::tera::Tera;
use rustdoc_types::{Crate, Id, ItemKind};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{RenderConfig, ModuleTree, GlobalItemIndex};
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
    /// Global item index for cross-crate linking (optional).
    pub global_index: Option<&'a GlobalItemIndex>,
}

impl<'a> RenderContext<'a> {
    /// Create a new render context.
    pub fn new(krate: &'a Crate, config: &'a RenderConfig) -> AnyResult<Self> {
        Self::new_with_index(krate, config, None)
    }

    /// Create a new render context with a global item index for cross-crate linking.
    pub fn new_with_index(
        krate: &'a Crate,
        config: &'a RenderConfig,
        global_index: impl Into<Option<&'a GlobalItemIndex>>,
    ) -> AnyResult<Self> {
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
            global_index: global_index.into(),
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

        // Check if this is a local item or a cross-crate item.
        if summary.crate_id == 0 {
            // Local item - use the path directly.
            self.build_item_url(&summary.path, summary.kind, current_depth)
        } else {
            // Cross-crate item - look up in global index.
            self.resolve_cross_crate_url(&summary.path, current_depth)
        }
    }

    /// Resolve a cross-crate item path to a URL using the global index.
    fn resolve_cross_crate_url(&self, path: &[String], current_depth: usize) -> Option<String> {
        let global_index = self.global_index?;
        let path_str = path.join("::");
        let location = global_index.items.get(&path_str)?;

        self.build_item_url(&location.path, location.kind, current_depth)
    }

    /// Build a URL for an item given its path and kind.
    fn build_item_url(&self, path: &[String], kind: ItemKind, current_depth: usize) -> Option<String> {
        let kind_prefix = match kind {
            ItemKind::Struct => "struct.",
            ItemKind::Enum => "enum.",
            ItemKind::Trait => "trait.",
            ItemKind::Function => "fn.",
            ItemKind::TypeAlias => "type.",
            ItemKind::Constant => "constant.",
            ItemKind::Static => "static.",
            ItemKind::Macro => "macro.",
            ItemKind::Module => "",
            _ => return None, // Don't link to other kinds.
        };

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

        if kind == ItemKind::Module {
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
