//! Types for organizing and rendering documentation.

use rmx::prelude::*;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, ItemKind, Type};
use std::collections::HashMap;
use std::path::PathBuf;

/// A renderable documentation item with computed paths.
#[derive(Debug, Clone)]
pub struct RenderableItem<'a> {
    /// The item's unique ID.
    pub id: &'a Id,
    /// The item data.
    pub item: &'a Item,
    /// Full module path (e.g., ["crate", "foo", "bar"]).
    pub path: Vec<String>,
    /// Output HTML file path relative to output root.
    pub html_path: PathBuf,
}

/// A node in the module tree hierarchy.
#[derive(Debug)]
pub struct ModuleTree<'a> {
    /// Module name.
    pub name: String,
    /// Module's own documentation item (None for the crate root pseudo-module).
    pub module_item: Option<RenderableItem<'a>>,
    /// Items directly contained in this module (functions, structs, etc.).
    pub items: Vec<RenderableItem<'a>>,
    /// Submodules.
    pub submodules: Vec<ModuleTree<'a>>,
    /// Glob re-exports from external crates (crate name -> target ID).
    pub glob_reexports: Vec<GlobReexport>,
}

/// A glob re-export from an external crate.
#[derive(Debug, Clone)]
pub struct GlobReexport {
    /// The source path of the glob import (e.g., "::tokio").
    pub source: String,
    /// The target crate name (e.g., "tokio").
    pub target_crate: String,
}

/// Builds a module tree from the rustdoc crate data.
pub fn build_module_tree<'a>(
    krate: &'a Crate,
    include_private: bool,
) -> AnyResult<ModuleTree<'a>> {
    let mut id_to_path: HashMap<&Id, Vec<String>> = HashMap::new();

    // Build path map from krate.paths.
    for (id, summary) in &krate.paths {
        id_to_path.insert(id, summary.path.clone());
    }

    // Get the root module.
    let root_item = krate.index.get(&krate.root)
        .ok_or_else(|| anyhow!("Root item not found in index"))?;

    let crate_name = root_item.name.clone().unwrap_or_else(|| "crate".to_string());

    // Build tree starting from root.
    build_tree_recursive(
        krate,
        &krate.root,
        vec![crate_name.clone()],
        include_private,
        &id_to_path,
    )
}

fn build_tree_recursive<'a>(
    krate: &'a Crate,
    module_id: &'a Id,
    current_path: Vec<String>,
    include_private: bool,
    id_to_path: &HashMap<&Id, Vec<String>>,
) -> AnyResult<ModuleTree<'a>> {
    let module_item = krate.index.get(module_id)
        .ok_or_else(|| anyhow!("Module {} not found in index", module_id.0))?;

    let module_name = module_item.name.clone()
        .unwrap_or_else(|| current_path.last().cloned().unwrap_or_default());

    let ItemEnum::Module(module) = &module_item.inner else {
        bail!("Expected module, got {:?}", module_item.inner);
    };

    let mut items = Vec::new();
    let mut submodules = Vec::new();
    let mut glob_reexports = Vec::new();

    for child_id in &module.items {
        let Some(child_item) = krate.index.get(child_id) else {
            continue;
        };

        // Skip private items unless requested.
        // But always include modules (they're navigable) and check re-exports by their target.
        if !include_private && is_private(child_item, krate) {
            continue;
        }

        // Handle glob re-exports specially.
        if let ItemEnum::Use(use_item) = &child_item.inner {
            if use_item.is_glob {
                // Extract crate name from source (e.g., "::tokio" -> "tokio").
                let target_crate = use_item.source
                    .trim_start_matches("::")
                    .split("::")
                    .next()
                    .unwrap_or("")
                    .to_string();

                if !target_crate.is_empty() {
                    glob_reexports.push(GlobReexport {
                        source: use_item.source.clone(),
                        target_crate,
                    });
                }
                continue;
            }
        }

        // Get child name - Use items store name in inner.use.name, not item.name.
        let child_name = if let ItemEnum::Use(use_item) = &child_item.inner {
            use_item.name.clone()
        } else {
            child_item.name.clone().unwrap_or_default()
        };
        let mut child_path = current_path.clone();
        child_path.push(child_name.clone());

        match &child_item.inner {
            ItemEnum::Module(_) => {
                // Recurse into submodule.
                let subtree = build_tree_recursive(
                    krate,
                    child_id,
                    child_path,
                    include_private,
                    id_to_path,
                )?;
                submodules.push(subtree);
            }
            _ => {
                // Regular item.
                let html_path = path_to_html(&child_path, item_kind(&child_item.inner));
                items.push(RenderableItem {
                    id: child_id,
                    item: child_item,
                    path: child_path,
                    html_path,
                });
            }
        }
    }

    // Sort items by name for consistent ordering.
    items.sort_by(|a, b| {
        a.item.name.as_deref().unwrap_or("")
            .cmp(b.item.name.as_deref().unwrap_or(""))
    });
    submodules.sort_by(|a, b| a.name.cmp(&b.name));

    let module_html_path = path_to_html(&current_path, Some(ItemKind::Module));
    let module_renderable = RenderableItem {
        id: module_id,
        item: module_item,
        path: current_path,
        html_path: module_html_path,
    };

    Ok(ModuleTree {
        name: module_name,
        module_item: Some(module_renderable),
        items,
        submodules,
        glob_reexports,
    })
}

/// Convert a module path to an HTML file path.
fn path_to_html(path: &[String], kind: Option<ItemKind>) -> PathBuf {
    if path.is_empty() {
        return PathBuf::from("index.html");
    }

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

fn item_kind(inner: &ItemEnum) -> Option<ItemKind> {
    match inner {
        ItemEnum::Module(_) => Some(ItemKind::Module),
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
    }
}

/// Check if an item should be considered private.
///
/// Modules are never private (they're always navigable).
/// Re-exports (Use items) check the target item's visibility.
/// Other items are private if they have `Visibility::Default`.
fn is_private(item: &Item, krate: &Crate) -> bool {
    use rustdoc_types::Visibility;

    // Modules are always public for navigation purposes.
    if matches!(item.inner, ItemEnum::Module(_)) {
        return false;
    }

    // For re-exports, check the target item's visibility.
    if let ItemEnum::Use(use_item) = &item.inner {
        if let Some(ref target_id) = use_item.id {
            if let Some(target_item) = krate.index.get(target_id) {
                // If target is a module, it's public.
                if matches!(target_item.inner, ItemEnum::Module(_)) {
                    return false;
                }
                // Check target's visibility.
                return matches!(target_item.visibility, Visibility::Default);
            }
        }
        // If we can't resolve the target, consider it public (it's an external re-export).
        return false;
    }

    matches!(item.visibility, Visibility::Default)
}

/// Information about an impl block.
#[derive(Debug, Clone)]
pub struct ImplInfo<'a> {
    /// The impl item ID.
    pub id: &'a Id,
    /// The impl data.
    pub impl_: &'a Impl,
    /// The type this impl is for.
    pub for_type: &'a Type,
    /// Trait being implemented (None for inherent impls).
    pub trait_path: Option<String>,
    /// Trait ID if this is a trait impl.
    pub trait_id: Option<&'a Id>,
}

/// Indices mapping types and traits to their implementations.
#[derive(Debug, Default)]
pub struct ImplIndex<'a> {
    /// Map from trait ID to implementations of that trait.
    pub trait_impls: HashMap<&'a Id, Vec<ImplInfo<'a>>>,
    /// Map from type ID to implementations for that type (both inherent and trait).
    pub type_impls: HashMap<&'a Id, Vec<ImplInfo<'a>>>,
}

/// Build an index of all impl blocks in the crate.
pub fn build_impl_index<'a>(krate: &'a Crate) -> ImplIndex<'a> {
    let mut index = ImplIndex::default();

    for (id, item) in &krate.index {
        let ItemEnum::Impl(impl_) = &item.inner else {
            continue;
        };

        // Get trait info if this is a trait impl.
        let (trait_path, trait_id) = if let Some(ref trait_) = impl_.trait_ {
            (Some(trait_.path.clone()), Some(&trait_.id))
        } else {
            (None, None)
        };

        let info = ImplInfo {
            id,
            impl_,
            for_type: &impl_.for_,
            trait_path,
            trait_id,
        };

        // Add to trait_impls if this is a trait impl.
        if let Some(trait_id) = trait_id {
            index.trait_impls
                .entry(trait_id)
                .or_default()
                .push(info.clone());
        }

        // Add to type_impls if we can identify the type's ID.
        if let Some(type_id) = get_type_id(&impl_.for_) {
            index.type_impls
                .entry(type_id)
                .or_default()
                .push(info);
        }
    }

    index
}

/// Try to extract the ID from a Type (works for resolved paths).
fn get_type_id(ty: &Type) -> Option<&Id> {
    match ty {
        Type::ResolvedPath(path) => Some(&path.id),
        _ => None,
    }
}
