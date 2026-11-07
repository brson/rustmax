//! Workspace-wide statistics for the summary pages.

use crate::index::{Index, SymId};
use crate::symbol::ScipSymbol;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Maps workspace files to the member package that owns them.
pub struct Owners {
    /// Package directories with a trailing `/`, or empty for a root package,
    /// longest first so nested packages win.
    dirs: Vec<(String, String)>,
}

impl Owners {
    /// Build from `(package directory, package name)` pairs.
    pub fn new<'a>(packages: impl IntoIterator<Item = (&'a str, &'a str)>) -> Owners {
        let mut dirs: Vec<(String, String)> = packages
            .into_iter()
            .map(|(dir, name)| {
                let prefix = if dir.is_empty() { String::new() } else { format!("{dir}/") };
                (prefix, name.to_string())
            })
            .collect();
        dirs.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        Owners { dirs }
    }

    /// The package owning `path`, if any.
    pub fn owner(&self, path: &str) -> Option<&str> {
        self.dirs.iter().find(|(prefix, _)| path.starts_with(prefix.as_str())).map(|(_, name)| name.as_str())
    }
}

/// How the workspace uses one external crate.
#[derive(Serialize)]
pub struct DepUsage {
    pub package: String,
    /// `None` for sysroot crates, whose SCIP version is a URL.
    pub version: Option<String>,
    pub docs_url: String,
    pub refs: usize,
    pub files: usize,
    /// Referencing workspace packages and their reference counts, most first.
    pub users: Vec<UserCount>,
    /// Referenced symbols, most referenced first.
    pub symbols: Vec<SymbolUsage>,
}

#[derive(Serialize)]
pub struct UserCount {
    pub package: String,
    pub refs: usize,
}

#[derive(Serialize)]
pub struct SymbolUsage {
    pub id: SymId,
    pub path: String,
    pub refs: usize,
}

/// Group references to symbols defined outside the workspace by defining crate.
///
/// Crates are grouped as rust-analyzer reports them, by the crate that defines
/// each item, so an item re-exported through a facade like `serde` counts
/// toward the crate it comes from, like `serde_core`.
pub fn dependency_usage(index: &Index, owners: &Owners, members: &BTreeSet<&str>) -> Vec<DepUsage> {
    struct Acc {
        version: String,
        refs: usize,
        files: BTreeSet<String>,
        users: HashMap<String, usize>,
        symbols: Vec<SymbolUsage>,
    }

    let mut by_package: BTreeMap<String, Acc> = BTreeMap::new();
    for (id, symbol) in index.symbols.iter().enumerate() {
        let Some(scip) = &symbol.scip else { continue };
        if symbol.def.is_some() || symbol.refs.is_empty() || members.contains(scip.package.as_str()) {
            continue;
        }
        let acc = by_package.entry(scip.package.clone()).or_insert_with(|| Acc {
            version: scip.version.clone(),
            refs: 0,
            files: BTreeSet::new(),
            users: HashMap::new(),
            symbols: Vec::new(),
        });
        acc.refs += symbol.refs.len();
        for r in &symbol.refs {
            acc.files.insert(r.path.clone());
            let owner = owners.owner(&r.path).unwrap_or("(none)");
            *acc.users.entry(owner.to_string()).or_default() += 1;
        }
        acc.symbols.push(SymbolUsage {
            id: id as SymId,
            path: symbol.display_path(),
            refs: symbol.refs.len(),
        });
    }

    let mut out: Vec<DepUsage> = by_package
        .into_iter()
        .map(|(package, mut acc)| {
            acc.symbols.sort_by(|a, b| b.refs.cmp(&a.refs).then_with(|| a.path.cmp(&b.path)));
            let mut users: Vec<UserCount> =
                acc.users.into_iter().map(|(package, refs)| UserCount { package, refs }).collect();
            users.sort_by(|a, b| b.refs.cmp(&a.refs).then_with(|| a.package.cmp(&b.package)));
            let crate_root = ScipSymbol { package: package.clone(), version: acc.version.clone(), descriptors: Vec::new() };
            DepUsage {
                docs_url: crate_root.docs_url(),
                version: (!acc.version.contains('/')).then_some(acc.version),
                package,
                refs: acc.refs,
                files: acc.files.len(),
                users,
                symbols: acc.symbols,
            }
        })
        .collect();
    out.sort_by(|a, b| b.refs.cmp(&a.refs).then_with(|| a.package.cmp(&b.package)));
    out
}

/// Counts of the items defined in one file.
#[derive(Serialize, Default, Clone)]
pub struct DefCounts {
    pub types: usize,
    pub traits: usize,
    pub functions: usize,
    pub constants: usize,
    pub macros: usize,
}

/// Definition counts per file, for symbols defined in the workspace.
pub fn def_counts(index: &Index) -> HashMap<&str, DefCounts> {
    let mut out: HashMap<&str, DefCounts> = HashMap::new();
    for symbol in &index.symbols {
        let (Some(def), Some(kind)) = (&symbol.def, &symbol.kind) else { continue };
        let counts = out.entry(def.path.as_str()).or_default();
        match kind.as_str() {
            "Struct" | "Enum" | "Union" | "TypeAlias" => counts.types += 1,
            "Trait" => counts.traits += 1,
            "Function" | "Method" | "StaticMethod" | "TraitMethod" => counts.functions += 1,
            "Constant" | "StaticVariable" | "AssociatedConst" => counts.constants += 1,
            "Macro" => counts.macros += 1,
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_packages_own_their_files() {
        let owners = Owners::new([("", "root"), ("crates/a", "a"), ("crates/a/sub", "sub")]);
        assert_eq!(owners.owner("crates/a/src/lib.rs"), Some("a"));
        assert_eq!(owners.owner("crates/a/sub/src/lib.rs"), Some("sub"));
        assert_eq!(owners.owner("crates/ab/src/lib.rs"), Some("root"));
        assert_eq!(owners.owner("src/main.rs"), Some("root"));
        let owners = Owners::new([("crates/a", "a")]);
        assert_eq!(owners.owner("README.md"), None);
    }
}
