//! Trait and impl relations from rustdoc JSON.
//!
//! rust-analyzer's SCIP output has no "implements" relationships,
//! so they come from rustdoc JSON, which is nightly-only.
//! rustdoc items are matched to SCIP symbols by source position:
//! the SCIP definition with the item's name inside the item's span.
//! Items from other workspace crates, which a crate's JSON knows only by path,
//! are matched by path instead.

use crate::index::{Index, Location, SymId, Target};
use crate::metadata::{Metadata, path_to_slash};
use rmx::prelude::*;
use rustmax_rustdoc::rustdoc_types::{Crate, Id, ItemEnum, Type};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Generate rustdoc JSON for each workspace target and add its relations to the index.
///
/// Returns the number of relations added.
pub fn add_relations(metadata: &Metadata, index: &mut Index) -> AnyResult<usize> {
    let target_dir = metadata.target_directory.join("rmxbrowse-rustdoc");
    let mut edits = Vec::new();
    for package in metadata.members() {
        for target in &package.targets {
            let selector: Vec<&str> = if target.kind.iter().any(|k| k.ends_with("lib") || k == "proc-macro") {
                vec!["--lib"]
            } else if target.kind.iter().any(|k| k == "bin") {
                vec!["--bin", &target.name]
            } else {
                continue;
            };
            eprintln!("Running rustdoc on {} ({})...", package.name, target.name);
            let output = Command::new("cargo")
                .arg(format!("+{}", rustmax_rustdoc::JSON_TOOLCHAIN))
                .arg("rustdoc")
                .arg("--manifest-path")
                .arg(&package.manifest_path)
                .arg("--target-dir")
                .arg(&target_dir)
                .args(&selector)
                .args(["--", "-Zunstable-options", "--output-format", "json", "--document-private-items"])
                .current_dir(&metadata.workspace_root)
                .output()
                .context("failed to run `cargo rustdoc`")?;
            if !output.status.success() {
                bail!(
                    "`cargo rustdoc` failed for {} ({}):\n{}",
                    package.name,
                    target.name,
                    String::from_utf8_lossy(&output.stderr),
                );
            }
            // Read each crate's JSON right away: targets with the same crate name,
            // like a binary and a library in different packages, write the same file.
            let json = target_dir.join("doc").join(format!("{}.json", target.name.replace('-', "_")));
            let krate = rustmax_rustdoc::load_json(&json)?;
            edits.extend(relations(index, &krate));
        }
    }
    Ok(apply(index, edits))
}

#[derive(Debug, PartialEq)]
enum Edit {
    /// An impl block for a type.
    Impl { ty: SymId, at: Location },
    /// An impl block of a trait.
    Implementation { tr: SymId, at: Location },
    /// An impl method implementing a trait method.
    Implements { method: SymId, trait_method: SymId },
}

fn apply(index: &mut Index, edits: Vec<Edit>) -> usize {
    let count = edits.len();
    for edit in edits {
        match edit {
            Edit::Impl { ty, at } => index.symbols[ty as usize].impls.push(at),
            Edit::Implementation { tr, at } => index.symbols[tr as usize].implementations.push(at),
            Edit::Implements { method, trait_method } => {
                index.symbols[method as usize].implements = Some(trait_method);
                if let Some(def) = index.symbols[method as usize].def.clone() {
                    index.symbols[trait_method as usize].implementations.push(def);
                }
            }
        }
    }
    for symbol in &mut index.symbols {
        for list in [&mut symbol.impls, &mut symbol.implementations] {
            list.sort();
            list.dedup();
        }
    }
    count
}

/// Lookups from rustdoc items to SCIP symbols.
struct Matcher<'a> {
    index: &'a Index,
    krate: &'a Crate,
    /// Global definitions in each file, as `(line, symbol)`, sorted.
    defs_by_file: HashMap<&'a str, Vec<(u32, SymId)>>,
    /// Workspace symbols by display path.
    by_path: HashMap<String, SymId>,
}

impl<'a> Matcher<'a> {
    fn new(index: &'a Index, krate: &'a Crate) -> Matcher<'a> {
        let mut defs_by_file: HashMap<&str, Vec<(u32, SymId)>> = HashMap::new();
        for (path, doc) in &index.documents {
            let defs = defs_by_file.entry(path.as_str()).or_default();
            for occ in doc.occurrences.iter().filter(|o| o.is_def) {
                if let Target::Global(id) = occ.target {
                    defs.push((occ.line, id));
                }
            }
            defs.sort();
        }
        let mut by_path = HashMap::new();
        for (id, symbol) in index.symbols.iter().enumerate() {
            if symbol.def.is_some() {
                by_path.entry(symbol.display_path()).or_insert(id as SymId);
            }
        }
        Matcher { index, krate, defs_by_file, by_path }
    }

    /// The workspace file and zero-based line where an item starts.
    fn location(&self, id: &Id) -> Option<Location> {
        let span = self.krate.index.get(id)?.span.as_ref()?;
        if span.filename.is_absolute() {
            return None;
        }
        let path = path_to_slash(&span.filename);
        self.index.documents.contains_key(&path).then(|| Location { path, line: span.begin.0 as u32 - 1 })
    }

    /// The SCIP symbol for a rustdoc item.
    fn symbol(&self, id: &Id) -> Option<SymId> {
        if let Some(item) = self.krate.index.get(id) {
            let name = item.name.as_deref()?;
            let span = item.span.as_ref()?;
            let path = path_to_slash(&span.filename);
            let (first, last) = (span.begin.0 as u32 - 1, span.end.0 as u32 - 1);
            let defs = self.defs_by_file.get(path.as_str())?;
            let start = defs.partition_point(|(line, _)| *line < first);
            return defs[start..]
                .iter()
                .take_while(|(line, _)| *line <= last)
                .map(|(_, sym)| *sym)
                .find(|sym| self.index.symbols[*sym as usize].name == name);
        }
        let summary = self.krate.paths.get(id)?;
        self.by_path.get(&summary.path.join("::")).copied()
    }
}

fn relations(index: &Index, krate: &Crate) -> Vec<Edit> {
    let m = Matcher::new(index, krate);
    let mut edits = Vec::new();
    for (impl_id, item) in &krate.index {
        let ItemEnum::Impl(imp) = &item.inner else { continue };
        if imp.is_synthetic || imp.blanket_impl.is_some() {
            continue;
        }
        let Some(at) = m.location(impl_id) else { continue };

        if let Type::ResolvedPath(path) = &imp.for_
            && let Some(ty) = m.symbol(&path.id)
        {
            edits.push(Edit::Impl { ty, at: at.clone() });
        }

        let Some(trait_path) = &imp.trait_ else { continue };
        let Some(tr) = m.symbol(&trait_path.id) else { continue };
        edits.push(Edit::Implementation { tr, at });

        let trait_display = index.symbols[tr as usize].display_path();
        for member_id in &imp.items {
            let Some(member) = krate.index.get(member_id) else { continue };
            let (ItemEnum::Function(_), Some(name)) = (&member.inner, &member.name) else { continue };
            let Some(method) = m.symbol(member_id) else { continue };
            if let Some(trait_method) = m.by_path.get(&format!("{trait_display}::{name}")) {
                edits.push(Edit::Implements { method, trait_method: *trait_method });
            }
        }
    }
    edits
}

/// Whether the pinned nightly is installed, for a clear error before a long run.
pub fn check_toolchain(workspace_root: &Path) -> AnyResult<()> {
    let output = Command::new("rustup")
        .args(["run", rustmax_rustdoc::JSON_TOOLCHAIN, "rustdoc", "--version"])
        .current_dir(workspace_root)
        .output()
        .context("failed to run `rustup`")?;
    if !output.status.success() {
        bail!(
            "toolchain {0} is required for --rustdoc; install it with `rustup toolchain install {0} --profile minimal`",
            rustmax_rustdoc::JSON_TOOLCHAIN,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax_rustdoc::rustdoc_types::{
        Abi, Function, FunctionHeader, FunctionSignature, Generics, Impl, Item, ItemKind, ItemSummary, Path as RdPath,
        Span, Struct, StructKind, Target as RdTarget, Trait, Visibility,
    };
    use scip::types::{Document as ScipDocument, Index as ScipIndex, Occurrence as ScipOccurrence, SymbolRole};

    // src/lib.rs, zero-based lines:
    //
    // 0 pub trait T {
    // 1     fn m(&self);
    // 2 }
    // 3 pub struct S;
    // 4 impl T for S {
    // 5     fn m(&self) {}
    // 6 }
    // 7 impl b::U for S {}
    const T: &str = "rust-analyzer cargo a 0.1.0 T#";
    const T_M: &str = "rust-analyzer cargo a 0.1.0 T#m().";
    const S: &str = "rust-analyzer cargo a 0.1.0 S#";
    const IMPL_M: &str = "rust-analyzer cargo a 0.1.0 impl#[S][T]m().";
    const U: &str = "rust-analyzer cargo b 0.1.0 U#";

    fn scip_index() -> Index {
        let occ = |line: i32, start: i32, end: i32, symbol: &str, is_def: bool| {
            let mut o = ScipOccurrence::new();
            o.range = vec![line, start, end];
            o.symbol = symbol.to_string();
            if is_def {
                o.symbol_roles = SymbolRole::Definition as i32;
            }
            o
        };
        let mut a = ScipDocument::new();
        a.relative_path = "src/lib.rs".to_string();
        a.occurrences = vec![
            occ(0, 10, 11, T, true),
            occ(1, 7, 8, T_M, true),
            occ(3, 11, 12, S, true),
            occ(4, 5, 6, T, false),
            occ(4, 11, 12, S, false),
            occ(5, 7, 8, IMPL_M, true),
            occ(7, 8, 9, U, false),
        ];
        let mut b = ScipDocument::new();
        b.relative_path = "b/src/lib.rs".to_string();
        b.occurrences = vec![occ(0, 10, 11, U, true)];
        let mut scip = ScipIndex::new();
        scip.documents = vec![a, b];
        Index::from_scip(&scip)
    }

    fn item(id: u32, name: Option<&str>, lines: (usize, usize), inner: ItemEnum) -> (Id, Item) {
        let item = Item {
            id: Id(id),
            crate_id: 0,
            name: name.map(str::to_string),
            span: Some(Span { filename: "src/lib.rs".into(), begin: (lines.0, 1), end: (lines.1, 1) }),
            visibility: Visibility::Public,
            docs: None,
            links: Default::default(),
            attrs: Vec::new(),
            deprecation: None,
            inner,
        };
        (Id(id), item)
    }

    fn generics() -> Generics {
        Generics { params: Vec::new(), where_predicates: Vec::new() }
    }

    fn method() -> ItemEnum {
        ItemEnum::Function(Function {
            sig: FunctionSignature { inputs: Vec::new(), output: None, is_c_variadic: false },
            generics: generics(),
            header: FunctionHeader { is_const: false, is_unsafe: false, is_async: false, abi: Abi::Rust },
            has_body: true,
        })
    }

    fn impl_of(trait_id: u32, items: Vec<Id>) -> ItemEnum {
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics: generics(),
            provided_trait_methods: Vec::new(),
            trait_: Some(RdPath { path: String::new(), id: Id(trait_id), args: None }),
            for_: Type::ResolvedPath(RdPath { path: "S".to_string(), id: Id(3), args: None }),
            items,
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        })
    }

    fn krate() -> Crate {
        let index = [
            item(1, Some("T"), (1, 3), ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                items: vec![Id(2)],
                generics: generics(),
                bounds: Vec::new(),
                implementations: vec![Id(4)],
            })),
            item(2, Some("m"), (2, 2), method()),
            item(3, Some("S"), (4, 4), ItemEnum::Struct(Struct {
                kind: StructKind::Unit,
                generics: generics(),
                impls: vec![Id(4), Id(6)],
            })),
            item(4, None, (5, 7), impl_of(1, vec![Id(5)])),
            item(5, Some("m"), (6, 6), method()),
            item(6, None, (8, 8), impl_of(7, Vec::new())),
        ];
        let paths = [(Id(7), ItemSummary { crate_id: 1, path: vec!["b".to_string(), "U".to_string()], kind: ItemKind::Trait })];
        Crate {
            root: Id(0),
            crate_version: None,
            includes_private: true,
            index: index.into_iter().collect(),
            paths: paths.into_iter().collect(),
            external_crates: Default::default(),
            target: RdTarget { triple: String::new(), target_features: Vec::new() },
            format_version: 0,
        }
    }

    #[test]
    fn matches_items_by_span_and_path() {
        let mut index = scip_index();
        let id = |index: &Index, s: &str| -> SymId {
            index.symbols.iter().position(|sym| sym.scip == crate::symbol::ScipSymbol::parse(s)).unwrap() as SymId
        };
        let edits = relations(&index, &krate());
        let at = |line| Location { path: "src/lib.rs".to_string(), line };

        assert!(edits.contains(&Edit::Impl { ty: id(&index, S), at: at(4) }));
        assert!(edits.contains(&Edit::Impl { ty: id(&index, S), at: at(7) }));
        assert!(edits.contains(&Edit::Implementation { tr: id(&index, T), at: at(4) }));
        assert!(edits.contains(&Edit::Implementation { tr: id(&index, U), at: at(7) }));
        assert!(edits.contains(&Edit::Implements { method: id(&index, IMPL_M), trait_method: id(&index, T_M) }));
        assert_eq!(edits.len(), 5);

        apply(&mut index, edits);
        let t_m = &index.symbols[id(&index, T_M) as usize];
        assert_eq!(t_m.implementations, [at(5)]);
        assert_eq!(index.symbols[id(&index, S) as usize].impls, [at(4), at(7)]);
        assert_eq!(index.symbols[id(&index, IMPL_M) as usize].implements, Some(id(&index, T_M)));
    }
}
