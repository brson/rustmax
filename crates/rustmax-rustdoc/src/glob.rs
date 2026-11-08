//! Resolving glob re-exports of other crates' modules.
//!
//! A glob like `pub use ::std::prelude::rust_2021::*` brings in the names
//! that module exports, which are themselves often re-exports and globs:
//! `rust_2021` is `pub use super::v1::*` plus `pub use core::prelude::rust_2021::*`.
//! Resolution follows these through the loaded crates' JSON,
//! applying Rust's rule that a name a module defines or imports by name
//! shadows the same name arriving through a glob.

use rustdoc_types::{Crate, Id, Item, ItemEnum, ItemKind, Visibility};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::types::item_kind;

/// The namespace a name lives in.
///
/// One name can be bound once per namespace,
/// as `alloc::vec` is both a module and a macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Namespace {
    Type,
    Value,
    Macro,
}

impl Namespace {
    pub fn of(kind: ItemKind) -> Namespace {
        match kind {
            ItemKind::Macro | ItemKind::ProcAttribute | ItemKind::ProcDerive => Namespace::Macro,
            ItemKind::Function | ItemKind::Constant | ItemKind::Static | ItemKind::Variant => {
                Namespace::Value
            }
            _ => Namespace::Type,
        }
    }
}

/// An item a glob re-export brings into scope.
#[derive(Debug, Clone)]
pub struct GlobItem<'a> {
    /// The name it is in scope under.
    pub name: String,
    /// The item's canonical path, starting with its crate name.
    pub path: Vec<String>,
    pub kind: ItemKind,
    /// The item itself, if its crate is loaded.
    pub item: Option<&'a Item>,
}

/// Resolves glob re-exports against a set of loaded crates.
pub struct GlobResolver<'a> {
    crates: &'a BTreeMap<String, Crate>,
    /// Per crate, its own items by canonical path, built on first use.
    by_path: HashMap<&'a str, HashMap<&'a [String], &'a Id>>,
}

impl<'a> GlobResolver<'a> {
    pub fn new(crates: &'a BTreeMap<String, Crate>) -> Self {
        GlobResolver { crates, by_path: HashMap::new() }
    }

    /// The items a glob of the module at `module_path` brings into scope.
    ///
    /// Empty if the module's crate is not loaded.
    pub fn glob_items(&mut self, module_path: &[String]) -> Vec<GlobItem<'a>> {
        let Some((crate_name, module_id)) = self.find_module(module_path) else {
            return Vec::new();
        };
        let mut expanding = HashSet::new();
        self.module_exports(crate_name, module_id, &mut expanding)
            .into_values()
            .collect()
    }

    /// Every public name a module exports, keyed by namespace and name.
    fn module_exports(
        &mut self,
        crate_name: &'a str,
        module_id: &'a Id,
        expanding: &mut HashSet<(&'a str, &'a Id)>,
    ) -> BTreeMap<(Namespace, String), GlobItem<'a>> {
        let mut exports = BTreeMap::new();
        if !expanding.insert((crate_name, module_id)) {
            return exports;
        }

        let krate = &self.crates[crate_name];
        let Some(ItemEnum::Module(module)) = krate.index.get(module_id).map(|i| &i.inner) else {
            panic!("glob target {crate_name} {module_id:?} is not a module");
        };

        // Names bound in the module itself, public or not, shadow its globs.
        // std relies on this to keep the `vec` module out of its prelude,
        // shadowing it with a private empty `mod vec {}`.
        let mut bound = HashSet::new();
        let mut globs = Vec::new();

        for child_id in &module.items {
            let Some(child) = krate.index.get(child_id) else {
                continue;
            };
            let public = child.visibility == Visibility::Public;

            let export = match &child.inner {
                ItemEnum::Use(use_item) if use_item.is_glob => {
                    if public {
                        globs.extend(use_item.id.as_ref());
                    }
                    continue;
                }
                ItemEnum::Use(use_item) => {
                    // A primitive re-export has no id, and `as _` binds no name.
                    let Some(target_id) = &use_item.id else { continue };
                    if use_item.name == "_" {
                        continue;
                    }
                    let Some(target) = self.target(krate, target_id) else { continue };
                    GlobItem { name: use_item.name.clone(), ..target }
                }
                inner => {
                    let (Some(name), Some(kind)) = (&child.name, item_kind(inner)) else {
                        continue;
                    };
                    bound.insert((Namespace::of(kind), name.clone()));
                    // Private items have no path, and only matter for shadowing.
                    let Some(summary) = krate.paths.get(child_id) else { continue };
                    GlobItem {
                        name: name.clone(),
                        path: summary.path.clone(),
                        kind,
                        item: Some(child),
                    }
                }
            };

            let key = (Namespace::of(export.kind), export.name.clone());
            bound.insert(key.clone());
            if public {
                exports.insert(key, export);
            }
        }

        for glob_id in globs {
            let Some((target_crate, target_module)) = self.glob_target(crate_name, glob_id) else {
                continue;
            };
            for (key, export) in self.module_exports(target_crate, target_module, expanding) {
                if !bound.contains(&key) {
                    exports.entry(key).or_insert(export);
                }
            }
        }

        expanding.remove(&(crate_name, module_id));
        exports
    }

    /// The module a glob in `crate_name` imports from.
    ///
    /// None for globs of enums, and of modules in crates that are not loaded.
    fn glob_target(&mut self, crate_name: &'a str, glob_id: &'a Id) -> Option<(&'a str, &'a Id)> {
        let krate = &self.crates[crate_name];
        if let Some(target) = krate.index.get(glob_id) {
            return matches!(target.inner, ItemEnum::Module(_)).then_some((crate_name, glob_id));
        }
        let summary = krate.paths.get(glob_id)?;
        if summary.kind != ItemKind::Module {
            return None;
        }
        self.find_module(&summary.path)
    }

    /// Describe the item a by-name re-export in `krate` points to.
    fn target(&mut self, krate: &'a Crate, target_id: &'a Id) -> Option<GlobItem<'a>> {
        let summary = krate.paths.get(target_id)?;
        let item = match krate.index.get(target_id) {
            Some(item) => Some(item),
            None => self.find_item(&summary.path),
        };
        Some(GlobItem {
            name: String::new(),
            path: summary.path.clone(),
            kind: summary.kind,
            item,
        })
    }

    /// Find a module by canonical path, in whichever loaded crate the path names.
    fn find_module(&mut self, path: &[String]) -> Option<(&'a str, &'a Id)> {
        let (crate_name, krate) = self.crates.get_key_value(path.first()?)?;
        let id = if path.len() == 1 {
            &krate.root
        } else {
            self.find_id(path)?
        };
        matches!(krate.index.get(id)?.inner, ItemEnum::Module(_)).then_some((crate_name.as_str(), id))
    }

    fn find_item(&mut self, path: &[String]) -> Option<&'a Item> {
        let krate = self.crates.get(path.first()?)?;
        krate.index.get(self.find_id(path)?)
    }

    fn find_id(&mut self, path: &[String]) -> Option<&'a Id> {
        let (crate_name, krate) = self.crates.get_key_value(path.first()?)?;
        let by_path = self.by_path.entry(crate_name.as_str()).or_insert_with(|| {
            krate.paths.iter()
                .filter(|(_, summary)| summary.crate_id == 0)
                .map(|(id, summary)| (summary.path.as_slice(), id))
                .collect()
        });
        by_path.get(path).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::*;

    /// Builds a crate's JSON, with ids local to it.
    struct CrateBuilder {
        krate: Crate,
    }

    impl CrateBuilder {
        fn new(name: &str) -> Self {
            let mut builder = CrateBuilder {
                krate: Crate {
                    root: Id(0),
                    crate_version: None,
                    includes_private: false,
                    index: Default::default(),
                    paths: Default::default(),
                    external_crates: Default::default(),
                    target: Target { triple: String::new(), target_features: vec![] },
                    format_version: 0,
                },
            };
            builder.add(0, Some(name), Visibility::Public, module(vec![]));
            builder.path(0, 0, &[name], ItemKind::Module);
            builder
        }

        fn add(&mut self, id: u32, name: Option<&str>, visibility: Visibility, inner: ItemEnum) {
            self.krate.index.insert(Id(id), Item {
                id: Id(id),
                crate_id: 0,
                name: name.map(String::from),
                span: None,
                visibility,
                docs: None,
                links: Default::default(),
                attrs: vec![],
                deprecation: None,
                inner,
            });
        }

        fn path(&mut self, id: u32, crate_id: u32, path: &[&str], kind: ItemKind) {
            self.krate.paths.insert(Id(id), ItemSummary {
                crate_id,
                path: path.iter().map(|s| s.to_string()).collect(),
                kind,
            });
        }

        /// Add a public item with its canonical path to module `parent`.
        fn define(&mut self, parent: u32, id: u32, path: &[&str], kind: ItemKind) {
            let inner = match kind {
                ItemKind::Module => module(vec![]),
                ItemKind::Macro => ItemEnum::Macro(String::new()),
                ItemKind::Struct => ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: Generics { params: vec![], where_predicates: vec![] },
                    impls: vec![],
                }),
                _ => unimplemented!(),
            };
            self.add(id, path.last().copied(), Visibility::Public, inner);
            self.path(id, 0, path, kind);
            self.child(parent, id);
        }

        /// Add a `use` to module `parent`.
        fn use_(&mut self, parent: u32, id: u32, name: &str, target: u32, is_glob: bool, visibility: Visibility) {
            self.add(id, None, visibility, ItemEnum::Use(Use {
                source: name.to_string(),
                name: name.to_string(),
                id: Some(Id(target)),
                is_glob,
            }));
            self.child(parent, id);
        }

        fn child(&mut self, parent: u32, id: u32) {
            let Some(ItemEnum::Module(module)) = self.krate.index.get_mut(&Id(parent)).map(|i| &mut i.inner) else {
                panic!("{parent} is not a module");
            };
            module.items.push(Id(id));
        }
    }

    fn module(items: Vec<u32>) -> ItemEnum {
        ItemEnum::Module(Module {
            is_crate: false,
            items: items.into_iter().map(Id).collect(),
            is_stripped: false,
        })
    }

    fn names(items: &[GlobItem]) -> Vec<String> {
        let mut names: Vec<String> = items.iter()
            .map(|i| format!("{:?} {} {}", i.kind, i.name, i.path.join("::")))
            .collect();
        names.sort();
        names
    }

    fn path(s: &str) -> Vec<String> {
        s.split("::").map(String::from).collect()
    }

    /// A glob of a module lists that module's exports, not its crate root's.
    #[test]
    fn glob_lists_named_module_only() {
        let mut b = CrateBuilder::new("std");
        b.define(0, 1, &["std", "fs"], ItemKind::Module);
        b.define(0, 2, &["std", "prelude"], ItemKind::Module);
        b.define(2, 3, &["std", "prelude", "Thing"], ItemKind::Struct);
        let crates = BTreeMap::from([("std".to_string(), b.krate)]);

        let items = GlobResolver::new(&crates).glob_items(&path("std::prelude"));

        assert_eq!(names(&items), ["Struct Thing std::prelude::Thing"]);
    }

    /// The std prelude shape: globs of a sibling module and of a module in
    /// another crate, and a private module whose own glob of the crate root
    /// is reached only through by-name imports.
    #[test]
    fn nested_globs_across_crates() {
        let mut core = CrateBuilder::new("core");
        core.define(0, 1, &["core", "option"], ItemKind::Module);
        core.define(1, 2, &["core", "option", "Option"], ItemKind::Struct);
        core.define(0, 3, &["core", "prelude"], ItemKind::Module);
        core.use_(3, 4, "Option", 2, false, Visibility::Public);

        let mut std = CrateBuilder::new("std");
        std.define(0, 1, &["std", "fs"], ItemKind::Module);
        std.define(0, 2, &["std", "vec"], ItemKind::Module);
        std.define(0, 3, &["std", "vec"], ItemKind::Macro);
        std.define(0, 4, &["std", "prelude"], ItemKind::Module);
        std.define(4, 5, &["std", "prelude", "v1"], ItemKind::Module);
        std.define(4, 6, &["std", "prelude", "rust_2021"], ItemKind::Module);
        // `v1`'s private `ambiguous_macros_only` globs the crate root.
        std.add(7, Some("ambiguous_macros_only"), Visibility::Crate, module(vec![]));
        std.child(5, 7);
        std.use_(7, 8, "crate", 0, true, Visibility::Public);
        std.use_(5, 9, "vec", 3, false, Visibility::Public);
        // `rust_2021` globs `v1` and core's prelude.
        std.use_(6, 10, "v1", 5, true, Visibility::Public);
        std.use_(6, 11, "prelude", 12, true, Visibility::Public);
        std.path(12, 1, &["core", "prelude"], ItemKind::Module);

        let crates = BTreeMap::from([
            ("core".to_string(), core.krate),
            ("std".to_string(), std.krate),
        ]);

        let mut resolver = GlobResolver::new(&crates);
        let items = resolver.glob_items(&path("std::prelude::rust_2021"));

        assert_eq!(names(&items), [
            "Macro vec std::vec",
            "Struct Option core::option::Option",
        ]);
        assert!(items.iter().all(|i| i.item.is_some()));
    }

    /// Names a module binds itself, even privately, shadow its globs,
    /// per namespace.
    #[test]
    fn bound_names_shadow_globs() {
        let mut b = CrateBuilder::new("std");
        b.define(0, 1, &["std", "vec"], ItemKind::Module);
        b.define(0, 2, &["std", "vec"], ItemKind::Macro);
        b.define(0, 3, &["std", "prelude"], ItemKind::Module);
        b.add(4, Some("vec"), Visibility::Crate, module(vec![]));
        b.child(3, 4);
        b.use_(3, 5, "crate", 0, true, Visibility::Public);
        let crates = BTreeMap::from([("std".to_string(), b.krate)]);

        let items = GlobResolver::new(&crates).glob_items(&path("std::prelude"));

        assert_eq!(names(&items), [
            "Macro vec std::vec",
            "Module prelude std::prelude",
        ]);
    }

    /// Modules globbing each other terminate.
    #[test]
    fn glob_cycle_terminates() {
        let mut b = CrateBuilder::new("c");
        b.define(0, 1, &["c", "a"], ItemKind::Module);
        b.define(0, 2, &["c", "b"], ItemKind::Module);
        b.define(1, 3, &["c", "a", "A"], ItemKind::Struct);
        b.use_(1, 4, "b", 2, true, Visibility::Public);
        b.use_(2, 5, "a", 1, true, Visibility::Public);
        let crates = BTreeMap::from([("c".to_string(), b.krate)]);

        let items = GlobResolver::new(&crates).glob_items(&path("c::a"));

        assert_eq!(names(&items), ["Struct A c::a::A"]);
    }

    #[test]
    fn unloaded_crate_resolves_to_nothing() {
        let crates = BTreeMap::new();
        assert!(GlobResolver::new(&crates).glob_items(&path("std::prelude")).is_empty());
    }
}
