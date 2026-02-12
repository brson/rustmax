//! Documentation rendering.

pub mod module;
pub mod item;
pub mod signature;
pub mod markdown;
pub mod highlight;
pub mod sidebar;

use rmx::prelude::*;
use rmx::tera::Tera;
use rustdoc_types::{Crate, Id, ItemEnum, ItemKind, Visibility};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

use crate::{RenderConfig, ModuleTree, GlobalItemIndex};
use crate::types::{build_module_tree, build_impl_index, get_type_id, ImplIndex};

/// Where a re-export target's page exists.
pub enum ReexportTarget {
    /// Target has a local public page.
    LocalPublic { path: Vec<String>, kind: ItemKind },
    /// Target has a page in an external crate we have docs for.
    External { path: Vec<String>, kind: ItemKind },
    /// Target needs its own page (private or external without docs).
    NeedsPage,
}

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
    /// All crates for resolving glob re-exports (optional, for multi-crate mode).
    pub all_crates: Option<&'a HashMap<String, Crate>>,
}

impl<'a> RenderContext<'a> {
    /// Create a new render context.
    pub fn new(krate: &'a Crate, config: &'a RenderConfig) -> AnyResult<Self> {
        Self::new_full(krate, config, None, None)
    }

    /// Create a new render context with a global item index for cross-crate linking.
    pub fn new_with_index(
        krate: &'a Crate,
        config: &'a RenderConfig,
        global_index: impl Into<Option<&'a GlobalItemIndex>>,
    ) -> AnyResult<Self> {
        Self::new_full(krate, config, global_index, None)
    }

    /// Create a new render context with full options.
    pub fn new_full(
        krate: &'a Crate,
        config: &'a RenderConfig,
        global_index: impl Into<Option<&'a GlobalItemIndex>>,
        all_crates: impl Into<Option<&'a HashMap<String, Crate>>>,
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
            all_crates: all_crates.into(),
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
    /// For local items, prefers re-export locations over internal definition paths.
    pub fn resolve_item_url(&self, id: &Id, current_depth: usize) -> Option<String> {
        // Delegate to resolve_reexport_url which handles re-exports, variants, and methods.
        self.resolve_reexport_url(id, current_depth)
    }

    /// Resolve a cross-crate item path to a URL using the global index.
    fn resolve_cross_crate_url(&self, path: &[String], current_depth: usize) -> Option<String> {
        let global_index = self.global_index?;
        let path_str = path.join("::");
        let location = global_index.items.get(&path_str)?;

        self.build_item_url(&location.path, location.kind, current_depth)
    }

    /// Build a URL for an item given its path and kind.
    pub fn build_item_url(&self, path: &[String], kind: ItemKind, current_depth: usize) -> Option<String> {
        let kind_prefix = match kind {
            ItemKind::Struct => "struct.",
            ItemKind::Union => "union.",
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

    /// Pre-resolve an item's `links` field into a URL map.
    ///
    /// Takes the item's `links` field (mapping link text to item IDs) and resolves
    /// each ID to an actual HTML URL. The keys are normalized to strip backtick
    /// wrappers since the markdown preprocessor strips them.
    pub fn resolve_item_links<S: BuildHasher>(
        &self,
        links: &std::collections::HashMap<String, Id, S>,
        current_depth: usize,
    ) -> HashMap<String, String> {
        let mut resolved = HashMap::new();
        for (text, id) in links {
            if let Some(url) = self.resolve_reexport_url(id, current_depth) {
                // The links field keys may have backtick wrappers (`` `Builder` ``).
                // Strip them since preprocess_shortcut_links removes them from URLs.
                let clean_key = text.trim_matches('`').to_string();
                resolved.insert(clean_key, url);
            }
        }
        resolved
    }

    /// Resolve an item ID to a URL, preferring the re-export location.
    ///
    /// Handles types, modules, methods, and enum variants.
    fn resolve_reexport_url(&self, id: &Id, current_depth: usize) -> Option<String> {
        // First check krate.paths (has types, modules, variants, but not methods).
        if let Some(summary) = self.krate.paths.get(id) {
            // Handle enum variants: link to parent enum page with #variant.Name anchor.
            if summary.kind == ItemKind::Variant {
                return self.resolve_variant_url(&summary.path, summary.crate_id, current_depth);
            }

            if summary.crate_id == 0 {
                // Local item. Check if it's re-exported to a public-facing location.
                if let Some(url) = self.find_reexport_url(id, summary.kind, current_depth) {
                    return Some(url);
                }
                // Fall back to definition path.
                return self.build_item_url(&summary.path, summary.kind, current_depth);
            } else {
                // Cross-crate item.
                return self.resolve_cross_crate_url(&summary.path, current_depth);
            }
        }

        // Not in paths - check if it's a method/associated item in the index.
        if let Some(item) = self.krate.index.get(id) {
            if matches!(item.inner, ItemEnum::Function(_)) {
                return self.resolve_method_url(id, item, current_depth);
            }
        }

        None
    }

    /// Resolve an enum variant to a URL with #variant.Name anchor.
    fn resolve_variant_url(
        &self,
        variant_path: &[String],
        crate_id: u32,
        current_depth: usize,
    ) -> Option<String> {
        if variant_path.len() < 2 {
            return None;
        }
        // Path is like ["core", "result", "Result", "Ok"].
        // Parent enum path is everything except last segment.
        let enum_path = &variant_path[..variant_path.len() - 1];
        let variant_name = variant_path.last()?;

        let enum_url = if crate_id == 0 {
            self.build_item_url(enum_path, ItemKind::Enum, current_depth)?
        } else {
            self.resolve_cross_crate_url(enum_path, current_depth)
                .or_else(|| self.build_item_url(enum_path, ItemKind::Enum, current_depth))?
        };

        Some(format!("{}#variant.{}", enum_url, variant_name))
    }

    /// Resolve a method/associated function to a URL with #method.name anchor.
    fn resolve_method_url(
        &self,
        method_id: &Id,
        method_item: &rustdoc_types::Item,
        current_depth: usize,
    ) -> Option<String> {
        let method_name = method_item.name.as_deref()?;

        // Find the parent type by searching impl blocks.
        for (_impl_id, impl_item) in &self.krate.index {
            let ItemEnum::Impl(impl_) = &impl_item.inner else {
                continue;
            };
            if !impl_.items.contains(method_id) {
                continue;
            }
            // Found the impl containing this method.
            // Get the type ID from the impl's for_ type.
            if let Some(type_id) = get_type_id(&impl_.for_) {
                if let Some(type_url) = self.resolve_reexport_url(type_id, current_depth) {
                    // Strip any existing fragment.
                    let base_url = type_url.split('#').next().unwrap_or(&type_url);
                    return Some(format!("{}#method.{}", base_url, method_name));
                }
            }
            break;
        }

        None
    }

    /// Find a re-export URL for a local item by checking if it appears as a
    /// Use target in the module tree.
    fn find_reexport_url(
        &self,
        target_id: &Id,
        kind: ItemKind,
        current_depth: usize,
    ) -> Option<String> {
        // Walk the module tree looking for Use items targeting this ID.
        self.find_reexport_in_tree(&self.module_tree, target_id, kind, current_depth)
    }

    fn find_reexport_in_tree(
        &self,
        tree: &crate::types::ModuleTree,
        target_id: &Id,
        kind: ItemKind,
        current_depth: usize,
    ) -> Option<String> {
        for item in &tree.items {
            if let ItemEnum::Use(use_item) = &item.item.inner {
                if use_item.id.as_ref() == Some(target_id) {
                    return self.build_item_url(&item.path, kind, current_depth);
                }
            }
        }
        for sub in &tree.submodules {
            if let Some(url) = self.find_reexport_in_tree(sub, target_id, kind, current_depth) {
                return Some(url);
            }
        }
        None
    }

    /// Render markdown to HTML.
    pub fn render_markdown(&self, md: &str) -> String {
        markdown::render_markdown(md, &self.highlighter)
    }

    /// Render markdown to HTML with intra-doc link resolution.
    pub fn render_markdown_with_links(&self, md: &str, current_depth: usize) -> String {
        let empty = HashMap::new();
        self.render_markdown_with_item_links(md, current_depth, &empty)
    }

    /// Render markdown to HTML with intra-doc link resolution and pre-resolved item links.
    pub fn render_markdown_with_item_links(
        &self,
        md: &str,
        current_depth: usize,
        pre_resolved_links: &HashMap<String, String>,
    ) -> String {
        markdown::render_markdown_with_links(
            md,
            &self.highlighter,
            self.global_index,
            self.crate_name(),
            current_depth,
            pre_resolved_links,
        )
    }

    /// Render a short documentation string (first paragraph only) as inline HTML.
    pub fn render_short_doc(&self, full_docs: &str, current_depth: usize) -> String {
        let empty = HashMap::new();
        self.render_short_doc_with_item_links(full_docs, current_depth, &empty)
    }

    /// Render a short doc string with pre-resolved item links.
    pub fn render_short_doc_with_item_links(
        &self,
        full_docs: &str,
        current_depth: usize,
        pre_resolved_links: &HashMap<String, String>,
    ) -> String {
        markdown::render_short_doc(
            full_docs,
            &self.highlighter,
            self.global_index,
            self.crate_name(),
            current_depth,
            pre_resolved_links,
        )
    }

    /// Determine where a re-export target's page exists.
    pub fn reexport_target(&self, target_id: &Id) -> ReexportTarget {
        let Some(summary) = self.krate.paths.get(target_id) else {
            return ReexportTarget::NeedsPage;
        };

        if summary.crate_id == 0 {
            // Local item - check if public.
            if let Some(target) = self.krate.index.get(target_id) {
                if target.visibility == Visibility::Public {
                    return ReexportTarget::LocalPublic {
                        path: summary.path.clone(),
                        kind: summary.kind,
                    };
                }
            }
        } else {
            // External item - check if we have docs for that crate.
            if let Some(all_crates) = self.all_crates {
                if let Some(crate_name) = summary.path.first() {
                    if all_crates.contains_key(crate_name) {
                        return ReexportTarget::External {
                            path: summary.path.clone(),
                            kind: summary.kind,
                        };
                    }
                }
            }
        }
        ReexportTarget::NeedsPage
    }
}

fn load_templates() -> AnyResult<Tera> {
    let mut tera = Tera::default();

    // Register templates from embedded strings.
    tera.add_raw_template("base.html", include_str!("../templates/base.html"))?;
    tera.add_raw_template("module.html", include_str!("../templates/module.html"))?;
    tera.add_raw_template("struct.html", include_str!("../templates/struct.html"))?;
    tera.add_raw_template("union.html", include_str!("../templates/union.html"))?;
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
