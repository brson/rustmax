//! The cross-reference index, built from rust-analyzer's SCIP output.

use crate::symbol::ScipSymbol;
use protobuf::Message;
use rmx::prelude::*;
use scip::types::{Index as ScipIndex, SymbolInformation, SymbolRole};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

pub type SymId = u32;

/// Every symbol and occurrence in the workspace.
pub struct Index {
    pub symbols: Vec<Symbol>,
    /// Documents by workspace-relative path.
    pub documents: HashMap<String, Document>,
}

/// A global symbol, defined in the workspace or outside it.
pub struct Symbol {
    pub scip: Option<ScipSymbol>,
    pub name: String,
    pub kind: Option<String>,
    pub signature: Option<String>,
    pub docs: Option<String>,
    pub def: Option<Location>,
    pub refs: Vec<Location>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub path: String,
    /// Zero-based line.
    pub line: u32,
}

/// The occurrences and local symbols of one source file.
#[derive(Default)]
pub struct Document {
    /// Single-line occurrences, sorted by position.
    pub occurrences: Vec<Occurrence>,
    pub locals: Vec<Local>,
}

pub struct Occurrence {
    pub line: u32,
    /// Byte offset of the start within the line.
    pub start: u32,
    /// Byte offset of the end within the line.
    pub end: u32,
    pub target: Target,
    pub is_def: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Global(SymId),
    /// An index into `Document::locals`.
    Local(u32),
}

/// A symbol local to one file, like a variable or parameter.
pub struct Local {
    pub name: String,
    pub signature: Option<String>,
    pub def_line: Option<u32>,
}

/// Run `rust-analyzer scip` over the workspace, writing the index to `scip_path`.
///
/// rust-analyzer logs internal errors liberally,
/// so its output goes to a `.log` file next to the index.
pub fn run_rust_analyzer(workspace_root: &Path, scip_path: &Path) -> AnyResult<()> {
    if let Some(dir) = scip_path.parent() {
        fs::create_dir_all(dir)?;
    }
    let log_path = scip_path.with_extension("log");
    let log = File::create(&log_path)
        .with_context(|| format!("failed to create {}", log_path.display()))?;
    let status = Command::new("rust-analyzer")
        .arg("scip")
        .arg(workspace_root)
        .arg("--output")
        .arg(scip_path)
        .stdout(log.try_clone()?)
        .stderr(log)
        .status()
        .context("failed to run `rust-analyzer`; install it with `rustup component add rust-analyzer`")?;
    if !status.success() {
        bail!("`rust-analyzer scip` failed, see {}", log_path.display());
    }
    Ok(())
}

impl Index {
    /// Load a SCIP index file.
    pub fn load(scip_path: &Path) -> AnyResult<Index> {
        let bytes = fs::read(scip_path)
            .with_context(|| format!("failed to read {}", scip_path.display()))?;
        let scip = ScipIndex::parse_from_bytes(&bytes)
            .with_context(|| format!("failed to parse SCIP index {}", scip_path.display()))?;
        Ok(Index::from_scip(&scip))
    }

    fn from_scip(scip: &ScipIndex) -> Index {
        let mut builder = Builder::default();

        // Symbol information first, so occurrences can find it.
        for doc in &scip.documents {
            for info in &doc.symbols {
                if !info.symbol.starts_with("local ") {
                    let id = builder.intern(&info.symbol);
                    builder.symbols[id as usize].set_info(info);
                }
            }
        }
        for info in &scip.external_symbols {
            let id = builder.intern(&info.symbol);
            builder.symbols[id as usize].set_info(info);
        }

        let mut documents = HashMap::new();
        for doc in &scip.documents {
            let path = doc.relative_path.clone();
            let mut document = Document::default();
            let mut locals: HashMap<&str, u32> = HashMap::new();
            for info in doc.symbols.iter().filter(|i| i.symbol.starts_with("local ")) {
                locals.insert(&info.symbol, document.locals.len() as u32);
                document.locals.push(Local {
                    name: info.display_name.clone(),
                    signature: signature_of(info),
                    def_line: None,
                });
            }

            for occ in &doc.occurrences {
                if occ.symbol.is_empty() {
                    continue;
                }
                let (line, start, end) = match occ.range[..] {
                    [line, start, end] => (line, start, end),
                    [line, start, end_line, end] if line == end_line => (line, start, end),
                    // Multi-line occurrences, like a module defined by a whole file,
                    // only contribute a definition location.
                    [line, ..] => (line, 0, 0),
                    _ => continue,
                };
                let (line, start, end) = (line as u32, start as u32, end as u32);
                let is_def = occ.symbol_roles & SymbolRole::Definition as i32 != 0;
                let location = Location { path: path.clone(), line };

                let target = if occ.symbol.starts_with("local ") {
                    let next = document.locals.len() as u32;
                    let local = *locals.entry(&occ.symbol).or_insert_with(|| next);
                    if local == next {
                        document.locals.push(Local {
                            name: String::new(),
                            signature: None,
                            def_line: None,
                        });
                    }
                    if is_def {
                        document.locals[local as usize].def_line.get_or_insert(line);
                    }
                    Target::Local(local)
                } else {
                    let id = builder.intern(&occ.symbol);
                    let symbol = &mut builder.symbols[id as usize];
                    if is_def {
                        symbol.def.get_or_insert(location);
                    } else {
                        symbol.refs.push(location);
                    }
                    Target::Global(id)
                };

                if end > start {
                    document.occurrences.push(Occurrence { line, start, end, target, is_def });
                }
            }
            document.occurrences.sort_by_key(|o| (o.line, o.start));
            documents.insert(path, document);
        }

        for symbol in &mut builder.symbols {
            symbol.refs.sort();
            symbol.refs.dedup();
        }

        Index { symbols: builder.symbols, documents }
    }
}

#[derive(Default)]
struct Builder {
    symbols: Vec<Symbol>,
    ids: HashMap<String, SymId>,
}

impl Builder {
    fn intern(&mut self, symbol: &str) -> SymId {
        if let Some(id) = self.ids.get(symbol) {
            return *id;
        }
        let id = self.symbols.len() as SymId;
        let scip = ScipSymbol::parse(symbol);
        let name = scip.as_ref().map(|s| s.name().to_string()).unwrap_or_else(|| symbol.to_string());
        self.symbols.push(Symbol {
            scip,
            name,
            kind: None,
            signature: None,
            docs: None,
            def: None,
            refs: Vec::new(),
        });
        self.ids.insert(symbol.to_string(), id);
        id
    }
}

impl Symbol {
    fn set_info(&mut self, info: &SymbolInformation) {
        if !info.display_name.is_empty() {
            self.name = info.display_name.clone();
        }
        self.kind = match info.kind.enum_value() {
            Ok(scip::types::symbol_information::Kind::UnspecifiedKind) | Err(_) => None,
            Ok(kind) => Some(format!("{kind:?}")),
        };
        self.signature = signature_of(info);
        let docs = info.documentation.join("\n\n");
        self.docs = (!docs.trim().is_empty()).then_some(docs);
    }

    /// Whether the symbol is defined in the workspace.
    pub fn is_local_to_workspace(&self) -> bool {
        self.def.is_some()
    }

    /// The display path, e.g. `rustmax_rustdoc::parse::load_json`.
    pub fn display_path(&self) -> String {
        self.scip.as_ref().map(|s| s.display_path()).unwrap_or_else(|| self.name.clone())
    }
}

fn signature_of(info: &SymbolInformation) -> Option<String> {
    info.signature_documentation
        .as_ref()
        .map(|d| d.text.clone())
        .filter(|t| !t.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use scip::types::symbol_information::Kind;
    use scip::types::{Document as ScipDocument, Occurrence as ScipOccurrence};

    fn occ(range: &[i32], symbol: &str, is_def: bool) -> ScipOccurrence {
        let mut o = ScipOccurrence::new();
        o.range = range.to_vec();
        o.symbol = symbol.to_string();
        if is_def {
            o.symbol_roles = SymbolRole::Definition as i32;
        }
        o
    }

    fn find<'a>(index: &'a Index, name: &str) -> &'a Symbol {
        index.symbols.iter().find(|s| s.name == name).unwrap()
    }

    #[test]
    fn builds_cross_references() {
        let krate = "rust-analyzer cargo a 0.1.0 crate/";
        let foo = "rust-analyzer cargo a 0.1.0 foo().";
        let path = "rust-analyzer cargo std https://x path/Path#";

        let mut info = SymbolInformation::new();
        info.symbol = foo.to_string();
        info.display_name = "foo".to_string();
        info.documentation = vec!["Does foo.".to_string()];
        info.kind = Kind::Function.into();
        let mut local = SymbolInformation::new();
        local.symbol = "local 0".to_string();
        local.display_name = "x".to_string();

        let mut a = ScipDocument::new();
        a.relative_path = "src/a.rs".to_string();
        a.symbols = vec![info, local];
        a.occurrences = vec![
            occ(&[2, 8, 12], path, false),
            occ(&[0, 0, 3, 0], krate, true),
            occ(&[1, 3, 6], foo, true),
            occ(&[1, 10, 11], "local 0", true),
            occ(&[2, 4, 5], "local 0", false),
        ];
        let mut b = ScipDocument::new();
        b.relative_path = "src/b.rs".to_string();
        b.occurrences = vec![occ(&[5, 0, 3], foo, false), occ(&[4, 2, 5], foo, false)];
        let mut scip = ScipIndex::new();
        scip.documents = vec![a, b];

        let index = Index::from_scip(&scip);

        let foo = find(&index, "foo");
        assert_eq!(foo.def, Some(Location { path: "src/a.rs".to_string(), line: 1 }));
        let ref_lines: Vec<(&str, u32)> = foo.refs.iter().map(|r| (r.path.as_str(), r.line)).collect();
        assert_eq!(ref_lines, [("src/b.rs", 4), ("src/b.rs", 5)]);
        assert_eq!(foo.kind.as_deref(), Some("Function"));
        assert_eq!(foo.docs.as_deref(), Some("Does foo."));
        assert!(foo.is_local_to_workspace());

        let path = find(&index, "Path");
        assert_eq!(path.def, None);
        assert_eq!(path.refs.len(), 1);
        assert_eq!(path.display_path(), "std::path::Path");

        // The crate's multi-line definition sets its location but is not linked.
        let krate = index.symbols.iter().find(|s| s.display_path() == "a").unwrap();
        assert_eq!(krate.def.as_ref().map(|d| d.line), Some(0));

        let doc = &index.documents["src/a.rs"];
        let positions: Vec<(u32, u32)> = doc.occurrences.iter().map(|o| (o.line, o.start)).collect();
        assert_eq!(positions, [(1, 3), (1, 10), (2, 4), (2, 8)]);
        assert_eq!(doc.locals.len(), 1);
        assert_eq!(doc.locals[0].name, "x");
        assert_eq!(doc.locals[0].def_line, Some(1));
    }
}
