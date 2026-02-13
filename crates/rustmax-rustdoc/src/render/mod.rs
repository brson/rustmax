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
            ItemKind::ProcDerive => "derive.",
            ItemKind::ProcAttribute => "attr.",
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
            // The links field keys may have backtick wrappers (`` `Builder` ``).
            // Strip them since preprocess_shortcut_links removes them from URLs.
            let clean_key = text.trim_matches('`').to_string();
            if let Some(url) = self.resolve_reexport_url(id, current_depth) {
                resolved.insert(clean_key, url);
            } else if let Some(url) = self.resolve_method_from_link_text(&clean_key, current_depth) {
                // Fallback: parse "Type::method" from the link text when the
                // method ID isn't in our index (common for cross-crate methods).
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
            // Find the enum's ID from paths and resolve through re-exports.
            let enum_id = self.krate.paths.iter()
                .find(|(_, s)| s.path == enum_path && s.kind == ItemKind::Enum)
                .map(|(id, _)| id);
            if let Some(enum_id) = enum_id {
                self.find_reexport_url(enum_id, ItemKind::Enum, current_depth)
                    .or_else(|| self.build_item_url(enum_path, ItemKind::Enum, current_depth))?
            } else {
                self.build_item_url(enum_path, ItemKind::Enum, current_depth)?
            }
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

        // Find the parent by searching impl blocks and trait definitions.
        for (parent_id, parent_item) in &self.krate.index {
            match &parent_item.inner {
                ItemEnum::Impl(impl_) => {
                    if !impl_.items.contains(method_id) {
                        continue;
                    }
                    // Found the impl containing this method.
                    if let Some(type_id) = get_type_id(&impl_.for_) {
                        if let Some(type_url) = self.resolve_reexport_url(type_id, current_depth) {
                            let base_url = type_url.split('#').next().unwrap_or(&type_url);
                            return Some(format!("{}#method.{}", base_url, method_name));
                        }
                    }
                    return None;
                }
                ItemEnum::Trait(trait_) => {
                    if !trait_.items.contains(method_id) {
                        continue;
                    }
                    // Found the trait containing this method.
                    if let Some(trait_url) = self.resolve_reexport_url(parent_id, current_depth) {
                        let base_url = trait_url.split('#').next().unwrap_or(&trait_url);
                        return Some(format!("{}#method.{}", base_url, method_name));
                    }
                    return None;
                }
                _ => continue,
            }
        }

        None
    }

    /// Resolve a "Type::method" pattern from link text.
    ///
    /// Fallback when the method ID isn't in our index (cross-crate methods).
    /// Parses the link text to find the parent type, resolves it, and appends
    /// a `#method.name` anchor.
    fn resolve_method_from_link_text(
        &self,
        text: &str,
        current_depth: usize,
    ) -> Option<String> {
        // Split "Type::method" or "mod::Type::method" on last "::".
        let (type_part, method_name) = text.rsplit_once("::")?;

        // Search krate.paths for the type by matching the last path segment,
        // preferring types/traits over modules.
        let type_name = type_part.rsplit("::").next().unwrap_or(type_part);
        let mut best: Option<(String, usize)> = None;

        for (id, summary) in &self.krate.paths {
            let is_type = matches!(
                summary.kind,
                ItemKind::Struct | ItemKind::Enum | ItemKind::Trait | ItemKind::Union
            );
            if !is_type {
                continue;
            }
            if summary.path.last().map(|s| s.as_str()) != Some(type_name) {
                continue;
            }
            if let Some(type_url) = self.resolve_reexport_url(id, current_depth) {
                let base_url = type_url.split('#').next().unwrap_or(&type_url);
                let url = format!("{}#method.{}", base_url, method_name);
                let path_len = summary.path.len();
                // Prefer shorter paths (more likely the canonical public location).
                if best.as_ref().map_or(true, |(_, len)| path_len < *len) {
                    best = Some((url, path_len));
                }
            }
        }

        best.map(|(url, _)| url)
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

/// A breadcrumb navigation entry.
#[derive(serde::Serialize)]
pub struct Breadcrumb {
    pub name: String,
    pub url: Option<String>,
}

/// Build breadcrumb entries for an item page.
///
/// For an item at path `["std", "thread", "spawn"]`, item pages live at
/// `std/thread/fn.spawn.html` (depth = path.len() - 1 = 2). Each ancestor
/// component links to its module's `index.html`.
pub fn build_breadcrumbs(path: &[String], depth: usize) -> Vec<Breadcrumb> {
    path.iter().enumerate().map(|(i, name)| {
        let url = if i == path.len() - 1 {
            None
        } else {
            // Go up `depth` levels to root, then down to the ancestor module.
            let ancestor_path = &path[..=i];
            Some(format!(
                "{}{}index.html",
                "../".repeat(depth),
                ancestor_path.iter()
                    .map(|p| format!("{}/", p))
                    .collect::<String>(),
            ))
        };
        Breadcrumb { name: name.clone(), url }
    }).collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::*;

    // Numeric IDs for test items.
    const ROOT: u32 = 0;
    const THREAD_MOD: u32 = 1;
    const JH_USE: u32 = 2;
    const SPAWN_USE: u32 = 3;
    const ENUM_USE: u32 = 4;
    const JH_DEF: u32 = 5;
    const SPAWN_DEF: u32 = 6;
    const ENUM_DEF: u32 = 7;
    const OK_VAR: u32 = 8;
    const ERR_VAR: u32 = 9;
    const TRAIT_DEF: u32 = 10;
    const TRAIT_METHOD: u32 = 11;
    const TRAIT_USE: u32 = 12;

    /// Helper to build a minimal Item.
    fn item(id: u32, name: &str, vis: Visibility, inner: ItemEnum) -> (Id, Item) {
        (Id(id), Item {
            id: Id(id),
            crate_id: 0,
            name: Some(name.to_string()),
            span: None,
            visibility: vis,
            docs: None,
            links: Default::default(),
            attrs: vec![],
            deprecation: None,
            inner,
        })
    }

    fn module(is_crate: bool, items: Vec<u32>) -> ItemEnum {
        ItemEnum::Module(Module {
            is_crate,
            items: items.into_iter().map(Id).collect(),
            is_stripped: false,
        })
    }

    fn use_item(source: &str, name: &str, target: u32) -> ItemEnum {
        ItemEnum::Use(Use {
            source: source.to_string(),
            name: name.to_string(),
            id: Some(Id(target)),
            is_glob: false,
        })
    }

    fn empty_struct() -> ItemEnum {
        ItemEnum::Struct(Struct {
            kind: StructKind::Unit,
            generics: Generics { params: vec![], where_predicates: vec![] },
            impls: vec![],
        })
    }

    fn empty_fn() -> ItemEnum {
        ItemEnum::Function(Function {
            sig: FunctionSignature {
                inputs: vec![], output: None, is_c_variadic: false,
            },
            generics: Generics { params: vec![], where_predicates: vec![] },
            header: FunctionHeader {
                is_const: false, is_unsafe: false, is_async: false, abi: Abi::Rust,
            },
            has_body: true,
        })
    }

    fn empty_enum(variant_ids: Vec<u32>) -> ItemEnum {
        ItemEnum::Enum(Enum {
            generics: Generics { params: vec![], where_predicates: vec![] },
            has_stripped_variants: false,
            variants: variant_ids.into_iter().map(Id).collect(),
            impls: vec![],
        })
    }

    fn plain_variant() -> ItemEnum {
        ItemEnum::Variant(Variant {
            kind: VariantKind::Plain,
            discriminant: None,
        })
    }

    fn empty_trait(method_ids: Vec<u32>) -> ItemEnum {
        ItemEnum::Trait(Trait {
            is_auto: false,
            is_unsafe: false,
            is_dyn_compatible: true,
            items: method_ids.into_iter().map(Id).collect(),
            generics: Generics { params: vec![], where_predicates: vec![] },
            bounds: vec![],
            implementations: vec![],
        })
    }

    fn path_entry(crate_id: u32, path: &[&str], kind: ItemKind) -> ItemSummary {
        ItemSummary {
            crate_id,
            path: path.iter().map(|s| s.to_string()).collect(),
            kind,
        }
    }

    /// Build a minimal crate with a re-export scenario:
    ///
    ///   mycrate
    ///   +-- thread (module)
    ///       +-- JoinHandle (re-export from _priv::JoinHandle)
    ///       +-- spawn (re-export from _priv::spawn)
    ///       +-- MyEnum (re-export from _priv::MyEnum)
    ///   +-- _priv (private module, not in public tree)
    ///       +-- JoinHandle (struct def)
    ///       +-- spawn (fn def)
    ///       +-- MyEnum (enum def)
    ///           +-- Ok (variant)
    ///           +-- Err (variant)
    fn test_crate() -> Crate {
        let mut krate = Crate {
            root: Id(ROOT),
            crate_version: None,
            includes_private: false,
            index: Default::default(),
            paths: Default::default(),
            external_crates: Default::default(),
            target: Target { triple: String::new(), target_features: vec![] },
            format_version: 0,
        };
        let index = &mut krate.index;
        let paths = &mut krate.paths;

        // Root module.
        let (id, i) = item(ROOT, "mycrate", Visibility::Public,
            module(true, vec![THREAD_MOD]));
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0, &["mycrate"], ItemKind::Module));

        // thread module.
        let (id, i) = item(THREAD_MOD, "thread", Visibility::Public,
            module(false, vec![JH_USE, SPAWN_USE, ENUM_USE, TRAIT_USE]));
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0, &["mycrate", "thread"], ItemKind::Module));

        // Re-export: JoinHandle.
        let (id, i) = item(JH_USE, "JoinHandle", Visibility::Public,
            use_item("mycrate::_priv::JoinHandle", "JoinHandle", JH_DEF));
        index.insert(id, i);

        // Re-export: spawn.
        let (id, i) = item(SPAWN_USE, "spawn", Visibility::Public,
            use_item("mycrate::_priv::spawn", "spawn", SPAWN_DEF));
        index.insert(id, i);

        // Re-export: MyEnum.
        let (id, i) = item(ENUM_USE, "MyEnum", Visibility::Public,
            use_item("mycrate::_priv::MyEnum", "MyEnum", ENUM_DEF));
        index.insert(id, i);

        // JoinHandle definition (in private module).
        let (id, i) = item(JH_DEF, "JoinHandle", Visibility::Public, empty_struct());
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "JoinHandle"], ItemKind::Struct));

        // spawn definition (in private module).
        let (id, i) = item(SPAWN_DEF, "spawn", Visibility::Public, empty_fn());
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "spawn"], ItemKind::Function));

        // MyEnum definition (in private module).
        let (id, i) = item(ENUM_DEF, "MyEnum", Visibility::Public,
            empty_enum(vec![OK_VAR, ERR_VAR]));
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "MyEnum"], ItemKind::Enum));

        // Enum variants.
        let (id, i) = item(OK_VAR, "Ok", Visibility::Public, plain_variant());
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "MyEnum", "Ok"], ItemKind::Variant));

        let (id, i) = item(ERR_VAR, "Err", Visibility::Public, plain_variant());
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "MyEnum", "Err"], ItemKind::Variant));

        // MyTrait definition (in private module) with one method.
        let (id, i) = item(TRAIT_DEF, "MyTrait", Visibility::Public,
            empty_trait(vec![TRAIT_METHOD]));
        index.insert(id.clone(), i);
        paths.insert(id, path_entry(0,
            &["mycrate", "_priv", "MyTrait"], ItemKind::Trait));

        // Trait method.
        let (id, i) = item(TRAIT_METHOD, "do_stuff", Visibility::Public, empty_fn());
        index.insert(id, i);

        // Re-export: MyTrait.
        let (id, i) = item(TRAIT_USE, "MyTrait", Visibility::Public,
            use_item("mycrate::_priv::MyTrait", "MyTrait", TRAIT_DEF));
        index.insert(id, i);

        krate
    }

    fn test_config() -> RenderConfig {
        RenderConfig {
            output_dir: PathBuf::from("/tmp/test"),
            ..Default::default()
        }
    }

    #[test]
    fn test_resolve_item_url_prefers_reexport() {
        let krate = test_crate();
        let config = test_config();
        let ctx = RenderContext::new(&krate, &config).unwrap();

        // JoinHandle is defined at mycrate::_priv::JoinHandle but re-exported
        // at mycrate::thread::JoinHandle. Should resolve to the re-export path.
        let url = ctx.resolve_item_url(&Id(JH_DEF), 2);
        assert_eq!(url, Some("../../mycrate/thread/struct.JoinHandle.html".to_string()));

        // spawn is defined at mycrate::_priv::spawn but re-exported
        // at mycrate::thread::spawn.
        let url = ctx.resolve_item_url(&Id(SPAWN_DEF), 2);
        assert_eq!(url, Some("../../mycrate/thread/fn.spawn.html".to_string()));
    }

    #[test]
    fn test_resolve_item_url_variant() {
        let krate = test_crate();
        let config = test_config();
        let ctx = RenderContext::new(&krate, &config).unwrap();

        // Enum variants should get anchor URLs on the re-exported enum page.
        let url = ctx.resolve_item_url(&Id(OK_VAR), 2);
        assert_eq!(url, Some("../../mycrate/thread/enum.MyEnum.html#variant.Ok".to_string()));

        let url = ctx.resolve_item_url(&Id(ERR_VAR), 0);
        assert_eq!(url, Some("mycrate/thread/enum.MyEnum.html#variant.Err".to_string()));
    }

    #[test]
    fn test_resolve_item_links() {
        let krate = test_crate();
        let config = test_config();
        let ctx = RenderContext::new(&krate, &config).unwrap();

        // Simulate an item's `links` field mapping "super::spawn" to the spawn ID.
        // Use the same FxHashMap type as Item.links by getting one from a dummy item.
        let (_, dummy) = item(99, "x", Visibility::Public, empty_struct());
        let mut links = dummy.links;
        links.insert("super::spawn".to_string(), Id(SPAWN_DEF));
        links.insert("super::JoinHandle".to_string(), Id(JH_DEF));

        let resolved = ctx.resolve_item_links(&links, 2);

        assert_eq!(
            resolved.get("super::spawn"),
            Some(&"../../mycrate/thread/fn.spawn.html".to_string()),
        );
        assert_eq!(
            resolved.get("super::JoinHandle"),
            Some(&"../../mycrate/thread/struct.JoinHandle.html".to_string()),
        );
    }

    #[test]
    fn test_resolve_trait_method_url() {
        let krate = test_crate();
        let config = test_config();
        let ctx = RenderContext::new(&krate, &config).unwrap();

        // Trait method do_stuff is defined inside MyTrait. It should resolve
        // to the trait page at the re-export location with a #method anchor.
        let url = ctx.resolve_reexport_url(&Id(TRAIT_METHOD), 2);
        assert_eq!(
            url,
            Some("../../mycrate/thread/trait.MyTrait.html#method.do_stuff".to_string()),
        );
    }

    #[test]
    fn test_build_breadcrumbs_item_page() {
        // Item at std::thread::spawn, depth=2 (file at std/thread/fn.spawn.html).
        let path: Vec<String> = vec!["std", "thread", "spawn"]
            .into_iter().map(String::from).collect();
        let crumbs = build_breadcrumbs(&path, 2);

        assert_eq!(crumbs.len(), 3);
        assert_eq!(crumbs[0].name, "std");
        assert_eq!(crumbs[0].url.as_deref(), Some("../../std/index.html"));
        assert_eq!(crumbs[1].name, "thread");
        assert_eq!(crumbs[1].url.as_deref(), Some("../../std/thread/index.html"));
        assert_eq!(crumbs[2].name, "spawn");
        assert_eq!(crumbs[2].url, None);
    }

    #[test]
    fn test_build_breadcrumbs_module_page() {
        // Module at std::thread, depth=2 (file at std/thread/index.html).
        let path: Vec<String> = vec!["std", "thread"]
            .into_iter().map(String::from).collect();
        let crumbs = build_breadcrumbs(&path, 2);

        assert_eq!(crumbs.len(), 2);
        assert_eq!(crumbs[0].name, "std");
        assert_eq!(crumbs[0].url.as_deref(), Some("../../std/index.html"));
        assert_eq!(crumbs[1].name, "thread");
        assert_eq!(crumbs[1].url, None);
    }

    #[test]
    fn test_build_breadcrumbs_crate_root() {
        // Crate root module, depth=1 (file at std/index.html).
        let path: Vec<String> = vec!["std"]
            .into_iter().map(String::from).collect();
        let crumbs = build_breadcrumbs(&path, 1);

        assert_eq!(crumbs.len(), 1);
        assert_eq!(crumbs[0].name, "std");
        assert_eq!(crumbs[0].url, None);
    }
}
