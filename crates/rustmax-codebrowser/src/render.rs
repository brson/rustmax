//! Page generation.

use crate::files::SourceFile;
use crate::graph;
use crate::highlight::{Highlighter, Link, escape};
use crate::index::{Document, Index, Location, SymId, Target};
use crate::metadata::{Metadata, Package};
use crate::outline;
use crate::summary::{self, DefCounts, Owners};
use rmx::prelude::*;
use rmx::rayon::prelude::*;
use rmx::serde_json::{self, json};
use rmx::tera::{Context, Tera};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

/// The directory under the output root holding everything that is not a mirrored page.
const ASSET_DIR: &str = "_rmx";

/// Symbols per lazily-loaded data file.
const SHARD_SIZE: u32 = 256;

/// The longest reference snippet shown, in bytes.
const MAX_SNIPPET: usize = 160;

/// Symbol kinds included in the search index.
const SEARCHABLE_KINDS: &[&str] = &[
    "Module", "Struct", "Enum", "EnumMember", "Union", "Trait", "TraitMethod", "Function",
    "Method", "StaticMethod", "Constant", "StaticVariable", "TypeAlias", "Macro", "Field",
    "AssociatedType", "AssociatedConst",
];

const BASE_TEMPLATE: &str = include_str!("../templates/base.html");
const SOURCE_TEMPLATE: &str = include_str!("../templates/source.html");
const DIR_TEMPLATE: &str = include_str!("../templates/dir.html");
const DEPS_TEMPLATE: &str = include_str!("../templates/deps.html");
const PACKAGE_TEMPLATE: &str = include_str!("../templates/package.html");
const BROWSE_CSS: &str = include_str!("../assets/browse.css");
const BROWSE_JS: &str = include_str!("../assets/browse.js");

/// Counts reported after a build.
#[derive(Debug, Default)]
pub struct Stats {
    pub files: usize,
    pub directories: usize,
    pub symbols: usize,
    pub workspace_symbols: usize,
}

pub struct Site<'a> {
    pub metadata: &'a Metadata,
    pub index: &'a Index,
    pub files: &'a [SourceFile],
    pub output: &'a Path,
}

struct Renderer<'a> {
    site: &'a Site<'a>,
    tera: Tera,
    highlighter: Highlighter,
    workspace_name: String,
    rendered: HashSet<&'a str>,
    /// Workspace member packages by directory.
    packages: BTreeMap<String, PackageView>,
    owners: Owners,
}

/// A package as shown on its directory page and manifest page.
#[derive(Serialize, Clone)]
struct PackageView {
    name: String,
    version: String,
    description: Option<String>,
    /// The package directory relative to the workspace root.
    dir: String,
    targets: Vec<TargetView>,
    deps: Vec<DepView>,
    features: Vec<FeatureView>,
}

#[derive(Serialize, Clone)]
struct TargetView {
    name: String,
    kind: String,
    /// Workspace-relative path to the target's page, without `.html`.
    path: Option<String>,
}

#[derive(Serialize, Clone)]
struct DepView {
    name: String,
    req: String,
    kind: String,
    optional: bool,
    resolved: Option<String>,
    /// Workspace-relative directory of a workspace member dependency.
    member_dir: Option<String>,
    docs_url: Option<String>,
}

#[derive(Serialize, Clone)]
struct FeatureView {
    name: String,
    enables: Vec<String>,
}

#[derive(Serialize)]
struct Crumb {
    name: String,
    url: Option<String>,
}

#[derive(Serialize)]
struct DirEntry {
    name: String,
    url: Option<String>,
    size: u64,
    lines: Option<usize>,
}

/// A Rust source file in a package's file table.
#[derive(Serialize)]
struct FileRow {
    /// The path relative to the package directory.
    name: String,
    url: String,
    lines: usize,
    defs: DefCounts,
}

#[derive(Serialize)]
struct PackageRow {
    name: String,
    version: String,
    description: Option<String>,
    url: String,
    rust_lines: usize,
}

pub fn render(site: &Site) -> AnyResult<Stats> {
    let mut tera = Tera::default();
    tera.add_raw_template("base.html", BASE_TEMPLATE)?;
    tera.add_raw_template("package.html", PACKAGE_TEMPLATE)?;
    tera.add_raw_template("source.html", SOURCE_TEMPLATE)?;
    tera.add_raw_template("dir.html", DIR_TEMPLATE)?;
    tera.add_raw_template("deps.html", DEPS_TEMPLATE)?;

    let workspace_name = site
        .metadata
        .workspace_root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "workspace".to_string());
    let rendered = site.files.iter().filter(|f| f.text.is_some()).map(|f| f.path.as_str()).collect();

    let mut renderer = Renderer {
        site,
        tera,
        highlighter: Highlighter::new(),
        workspace_name,
        rendered,
        packages: BTreeMap::new(),
        owners: Owners::new([]),
    };
    renderer.packages = renderer.package_views()?;
    renderer.owners = Owners::new(renderer.packages.values().map(|p| (p.dir.as_str(), p.name.as_str())));

    let assets = site.output.join(ASSET_DIR);
    fs::create_dir_all(assets.join("sym"))?;
    fs::write(assets.join("browse.css"), BROWSE_CSS)?;
    fs::write(assets.join("browse.js"), BROWSE_JS)?;
    fs::write(assets.join("rustmax-themes.css"), rustmax_rustdoc::assets::THEMES_CSS)?;
    fs::write(assets.join("rustmax-syntax.css"), rustmax_rustdoc::assets::SYNTAX_CSS)?;

    site.files
        .par_iter()
        .filter(|f| f.text.is_some())
        .try_for_each(|f| renderer.write_source_page(f))?;
    let directories = renderer.write_dir_pages()?;
    renderer.write_deps_page()?;
    renderer.write_symbol_shards()?;
    renderer.write_search_index()?;

    Ok(Stats {
        files: renderer.rendered.len(),
        directories,
        symbols: site.index.symbols.len(),
        workspace_symbols: site.index.symbols.iter().filter(|s| s.is_local_to_workspace()).count(),
    })
}

impl<'a> Renderer<'a> {
    fn package_views(&self) -> AnyResult<BTreeMap<String, PackageView>> {
        let metadata = self.site.metadata;
        let root = &metadata.workspace_root;
        let members = metadata.members();
        let member_dirs: HashMap<&str, String> = members
            .iter()
            .map(|p| Ok((p.name.as_str(), p.rel_dir(root)?)))
            .collect::<AnyResult<_>>()?;
        let resolved = metadata.resolved_deps();

        let mut views = BTreeMap::new();
        for package in members {
            let dir = member_dirs[package.name.as_str()].clone();
            let resolved = resolved.get(package.id.as_str());
            let targets = package
                .targets
                .iter()
                .map(|t| TargetView {
                    name: t.name.clone(),
                    kind: t.kind.join(", "),
                    path: t
                        .src_path
                        .strip_prefix(root)
                        .ok()
                        .map(crate::metadata::path_to_slash)
                        .filter(|p| self.rendered.contains(p.as_str())),
                })
                .collect();
            let mut deps: Vec<DepView> = package
                .dependencies
                .iter()
                .map(|d| {
                    let resolved: Option<&Package> = resolved.and_then(|r| r.get(d.name.as_str()).copied());
                    let member_dir = member_dirs.get(d.name.as_str()).cloned();
                    let version = resolved.map(|p| p.version.clone());
                    let docs_url = match (&member_dir, &version) {
                        (None, Some(v)) => Some(format!("https://docs.rs/{}/{}", d.name, v)),
                        _ => None,
                    };
                    let mut kind = d.kind.clone().unwrap_or_else(|| "normal".to_string());
                    if let Some(target) = &d.target {
                        kind = format!("{kind}, {target}");
                    }
                    DepView {
                        name: d.name.clone(),
                        req: d.req.clone(),
                        kind,
                        optional: d.optional,
                        resolved: version,
                        member_dir,
                        docs_url,
                    }
                })
                .collect();
            deps.sort_by(|a, b| (&a.kind, &a.name).cmp(&(&b.kind, &b.name)));
            let features = package
                .features
                .iter()
                .map(|(name, enables)| FeatureView { name: name.clone(), enables: enables.clone() })
                .collect();
            views.insert(dir.clone(), PackageView {
                name: package.name.clone(),
                version: package.version.clone(),
                description: package.description.clone(),
                dir,
                targets,
                deps,
                features,
            });
        }
        Ok(views)
    }

    /// Context shared by every page.
    ///
    /// `path` is the page's mirrored path: a file path, or a directory path for listings.
    fn base_context(&self, path: &str, is_dir: bool) -> Context {
        let components: Vec<&str> = path.split('/').filter(|c| !c.is_empty()).collect();
        let depth = if is_dir { components.len() } else { components.len().saturating_sub(1) };
        let root = "../".repeat(depth);

        let mut crumbs = vec![Crumb {
            name: self.workspace_name.clone(),
            url: Some(format!("{root}index.html")),
        }];
        for (i, name) in components.iter().enumerate() {
            let last = i + 1 == components.len();
            let url = if last {
                None
            } else {
                Some(format!("{root}{}/index.html", components[..=i].join("/")))
            };
            crumbs.push(Crumb { name: name.to_string(), url });
        }

        let mut ctx = Context::new();
        ctx.insert("root", &root);
        ctx.insert("assets", &format!("{root}{ASSET_DIR}/"));
        ctx.insert("workspace_name", &self.workspace_name);
        ctx.insert("crumbs", &crumbs);
        ctx.insert("title", components.last().copied().unwrap_or(&self.workspace_name));
        ctx
    }

    fn write_page(&self, rel: &str, html: &str) -> AnyResult<()> {
        let path = self.site.output.join(rel);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))
    }

    fn write_source_page(&self, file: &SourceFile) -> AnyResult<()> {
        let text = file.text.as_deref().expect("rendered files have text");
        let mut ctx = self.base_context(&file.path, false);
        let root: String = "../".repeat(file.path.matches('/').count());

        let document = self.site.index.documents.get(&file.path);
        let links = document.map(|d| self.links_for(&file.path, &root, d)).unwrap_or_default();
        let syntax = self.highlighter.syntax_for(file.name(), text);
        let lines = self.highlighter.highlight_lines(text, syntax, &links)?;

        let mut code = String::new();
        for (i, line) in lines.iter().enumerate() {
            let n = i + 1;
            write!(code, "<span class=\"rb-line\" id=\"L{n}\"><a class=\"rb-ln\" href=\"#L{n}\" data-n=\"{n}\"></a>{line}</span>")?;
        }

        let locals: Vec<serde_json::Value> = document
            .map(|d| {
                d.locals
                    .iter()
                    .map(|l| json!({ "n": l.name, "sig": l.signature }))
                    .collect()
            })
            .unwrap_or_default();
        // Escape `<` so the JSON cannot close the script element.
        let locals = serde_json::to_string(&locals)?.replace('<', "\\u003c");

        let outline = document
            .map(|d| outline::build(d, &self.site.index.symbols))
            .filter(|items| !items.is_empty())
            .map(|items| outline::render_html(&items));

        let package = (file.name() == "Cargo.toml").then(|| self.packages.get(file.dir())).flatten();

        ctx.insert("code", &code);
        ctx.insert("locals", &locals);
        ctx.insert("outline", &outline);
        ctx.insert("syntax", &syntax.name);
        ctx.insert("line_count", &lines.len());
        ctx.insert("size", &file.size);
        ctx.insert("indexed", &document.is_some());
        ctx.insert("package", &package);
        let html = self.tera.render("source.html", &ctx)?;
        self.write_page(&format!("{}.html", file.path), &html)
    }

    /// Hyperlinks for every occurrence in a document.
    fn links_for(&self, path: &str, root: &str, document: &Document) -> Vec<Link> {
        let symbols = &self.site.index.symbols;
        document
            .occurrences
            .iter()
            .map(|occ| {
                let attrs = match occ.target {
                    Target::Global(id) => {
                        let symbol = &symbols[id as usize];
                        if occ.is_def {
                            format!("class=\"d\" data-s=\"{id}\" href=\"#L{}\"", occ.line + 1)
                        } else if let Some(def) = &symbol.def {
                            match self.page_href(root, path, &def.path) {
                                Some(href) => format!("class=\"r\" data-s=\"{id}\" href=\"{href}#L{}\"", def.line + 1),
                                None => format!("class=\"r\" data-s=\"{id}\""),
                            }
                        } else if let Some(scip) = &symbol.scip {
                            format!("class=\"x\" data-s=\"{id}\" href=\"{}\"", escape(&scip.docs_url()))
                        } else {
                            format!("class=\"x\" data-s=\"{id}\"")
                        }
                    }
                    Target::Local(local) => {
                        let def_line = document.locals[local as usize].def_line;
                        let class = if occ.is_def { "v d" } else { "v" };
                        match def_line {
                            Some(line) => format!("class=\"{class}\" data-l=\"{local}\" href=\"#L{}\"", line + 1),
                            None => format!("class=\"{class}\" data-l=\"{local}\""),
                        }
                    }
                };
                Link { line: occ.line, start: occ.start, end: occ.end, attrs }
            })
            .collect()
    }

    /// The href from the page for `from` to the page for `to`, if `to` was rendered.
    fn page_href(&self, root: &str, from: &str, to: &str) -> Option<String> {
        if !self.rendered.contains(to) {
            return None;
        }
        if from == to {
            return Some(String::new());
        }
        Some(format!("{root}{}.html", escape(to)))
    }

    fn write_dir_pages(&self) -> AnyResult<usize> {
        let def_counts = summary::def_counts(self.site.index);
        let mut dirs: BTreeMap<&str, (BTreeSet<&str>, Vec<&SourceFile>)> = BTreeMap::new();
        dirs.entry("").or_default();
        for file in self.site.files {
            let dir = file.dir();
            dirs.entry(dir).or_default().1.push(file);
            // Register each ancestor, and each directory with its parent.
            let mut child = dir;
            while !child.is_empty() {
                let (parent, name) = child.rsplit_once('/').unwrap_or(("", child));
                let entry = dirs.entry(parent).or_default();
                if !entry.0.insert(name) {
                    break;
                }
                child = parent;
            }
        }

        for (dir, (subdirs, files)) in &dirs {
            let mut ctx = self.base_context(dir, true);
            let subdirs: Vec<DirEntry> = subdirs
                .iter()
                .map(|name| DirEntry {
                    name: format!("{name}/"),
                    url: Some(format!("{name}/index.html")),
                    size: 0,
                    lines: None,
                })
                .collect();
            let files: Vec<DirEntry> = files
                .iter()
                .map(|f| DirEntry {
                    name: f.name().to_string(),
                    url: f.text.as_ref().map(|_| format!("{}.html", f.name())),
                    size: f.size,
                    lines: f.text.as_ref().map(|t| t.lines().count()),
                })
                .collect();
            ctx.insert("subdirs", &subdirs);
            ctx.insert("files", &files);
            let package = self.packages.get(*dir);
            ctx.insert("package", &package);
            if let Some(package) = package {
                ctx.insert("package_files", &self.package_files(package, &def_counts));
            }
            ctx.insert("is_root", &dir.is_empty());
            if dir.is_empty() {
                ctx.insert("graph", &self.crate_graph());
                ctx.insert("packages", &self.package_rows());
                ctx.insert("file_count", &self.site.files.len());
                ctx.insert("symbol_count", &self.site.index.symbols.iter().filter(|s| s.is_local_to_workspace()).count());
            }
            let html = self.tera.render("dir.html", &ctx)?;
            let page = if dir.is_empty() { "index.html".to_string() } else { format!("{dir}/index.html") };
            self.write_page(&page, &html)?;
        }
        Ok(dirs.len())
    }

    fn package_rows(&self) -> Vec<PackageRow> {
        self.packages
            .values()
            .map(|p| {
                let rust_lines = self
                    .rust_files_of(p)
                    .filter_map(|f| f.text.as_ref())
                    .map(|t| t.lines().count())
                    .sum();
                PackageRow {
                    name: p.name.clone(),
                    version: p.version.clone(),
                    description: p.description.clone(),
                    url: dir_index(&p.dir),
                    rust_lines,
                }
            })
            .collect()
    }

    /// The rendered Rust files owned by a package, excluding nested packages.
    fn rust_files_of<'s>(&'s self, package: &'s PackageView) -> impl Iterator<Item = &'s SourceFile> {
        self.site.files.iter().filter(move |f| {
            f.text.is_some() && f.path.ends_with(".rs") && self.owners.owner(&f.path) == Some(package.name.as_str())
        })
    }

    fn package_files(&self, package: &PackageView, def_counts: &HashMap<&str, DefCounts>) -> Vec<FileRow> {
        let prefix_len = if package.dir.is_empty() { 0 } else { package.dir.len() + 1 };
        self.rust_files_of(package)
            .map(|f| FileRow {
                name: f.path[prefix_len..].to_string(),
                url: format!("{}.html", &f.path[prefix_len..]),
                lines: f.text.as_ref().map(|t| t.lines().count()).unwrap_or(0),
                defs: def_counts.get(f.path.as_str()).cloned().unwrap_or_default(),
            })
            .collect()
    }

    /// The SVG graph of dependencies among workspace members, for the root page.
    fn crate_graph(&self) -> String {
        let packages: Vec<&PackageView> = self.packages.values().collect();
        let index_of: HashMap<&str, usize> =
            packages.iter().enumerate().map(|(i, p)| (p.dir.as_str(), i)).collect();
        let nodes: Vec<graph::Node> = packages
            .iter()
            .map(|p| graph::Node { name: p.name.clone(), href: dir_index(&p.dir) })
            .collect();
        // One edge per pair, dev only if every dependency between them is dev.
        let mut pairs: BTreeMap<(usize, usize), bool> = BTreeMap::new();
        for (from, p) in packages.iter().enumerate() {
            for d in &p.deps {
                let Some(to) = d.member_dir.as_deref().and_then(|dir| index_of.get(dir)) else { continue };
                let dev = d.kind.starts_with("dev");
                let entry = pairs.entry((from, *to)).or_insert(dev);
                *entry &= dev;
            }
        }
        let edges: Vec<graph::Edge> =
            pairs.into_iter().map(|((from, to), dev)| graph::Edge { from, to, dev }).collect();
        graph::render_svg(&nodes, &edges)
    }

    fn write_deps_page(&self) -> AnyResult<()> {
        let members: BTreeSet<&str> = self.packages.values().map(|p| p.name.as_str()).collect();
        let usage = summary::dependency_usage(self.site.index, &self.owners, &members);
        let page = format!("{ASSET_DIR}/deps.html");
        let mut ctx = self.base_context(&page, false);
        ctx.insert("crumbs", &[
            Crumb { name: self.workspace_name.clone(), url: Some("../index.html".to_string()) },
            Crumb { name: "dependency usage".to_string(), url: None },
        ]);
        ctx.insert("title", "dependency usage");
        ctx.insert("deps", &usage);
        let html = self.tera.render("deps.html", &ctx)?;
        self.write_page(&page, &html)
    }

    /// Write per-symbol data, sharded, as JS files so pages work from `file://` URLs.
    fn write_symbol_shards(&self) -> AnyResult<()> {
        let lines: HashMap<&str, Vec<&str>> = self
            .site
            .files
            .iter()
            .filter_map(|f| f.text.as_deref().map(|t| (f.path.as_str(), t.lines().collect())))
            .collect();
        let symbols = &self.site.index.symbols;
        let shard_count = (symbols.len() as u32).div_ceil(SHARD_SIZE);

        (0..shard_count).into_par_iter().try_for_each(|shard| {
            let start = shard * SHARD_SIZE;
            let end = (start + SHARD_SIZE).min(symbols.len() as u32);
            let mut entries = serde_json::Map::new();
            for id in start..end {
                entries.insert(id.to_string(), self.symbol_json(id, &lines)?);
            }
            let json = serde_json::to_string(&entries)?;
            let path = self.site.output.join(ASSET_DIR).join("sym").join(format!("{shard}.js"));
            fs::write(&path, format!("rmxSymbols({shard},{json});\n"))
                .with_context(|| format!("failed to write {}", path.display()))
        })
    }

    fn symbol_json(&self, id: SymId, lines: &HashMap<&str, Vec<&str>>) -> AnyResult<serde_json::Value> {
        let symbol = &self.site.index.symbols[id as usize];
        let signature = match &symbol.signature {
            Some(sig) => Some(self.highlighter.highlight_rust(sig)?),
            None => None,
        };
        let docs = symbol.docs.as_deref().map(markdown_to_html);
        let def = symbol.def.as_ref().and_then(|d| self.def_href(d));
        let docs_url = match (&symbol.def, &symbol.scip) {
            (None, Some(scip)) => Some(scip.docs_url()),
            _ => None,
        };
        let locations = |locs: &[Location]| -> Vec<serde_json::Value> {
            locs.iter()
                .map(|r| {
                    let snippet = lines
                        .get(r.path.as_str())
                        .and_then(|l| l.get(r.line as usize))
                        .map(|l| truncate(l.trim(), MAX_SNIPPET))
                        .unwrap_or("");
                    json!([r.path, r.line + 1, snippet])
                })
                .collect()
        };
        let implements = symbol.implements.map(|t| {
            let t = &self.site.index.symbols[t as usize];
            json!({ "p": t.display_path(), "def": t.def.as_ref().and_then(|d| self.def_href(d)) })
        });
        let package = symbol.scip.as_ref().map(|s| {
            if s.version.contains('/') { s.package.clone() } else { format!("{} {}", s.package, s.version) }
        });
        Ok(json!({
            "n": symbol.name,
            "p": symbol.display_path(),
            "k": symbol.kind,
            "pkg": package,
            "sig": signature,
            "doc": docs,
            "def": def,
            "url": docs_url,
            "refs": locations(&symbol.refs),
            "impls": locations(&symbol.impls),
            "imps": locations(&symbol.implementations),
            "implements": implements,
        }))
    }

    /// The root-relative href of a definition, if its file was rendered.
    fn def_href(&self, def: &Location) -> Option<String> {
        self.rendered.contains(def.path.as_str()).then(|| format!("{}.html#L{}", def.path, def.line + 1))
    }

    /// Write the symbol and file name search index.
    fn write_search_index(&self) -> AnyResult<()> {
        let mut entries: Vec<serde_json::Value> = Vec::new();
        for (id, symbol) in self.site.index.symbols.iter().enumerate() {
            let Some(def) = &symbol.def else { continue };
            let Some(kind) = &symbol.kind else { continue };
            if !SEARCHABLE_KINDS.contains(&kind.as_str()) || !self.rendered.contains(def.path.as_str()) {
                continue;
            }
            entries.push(json!([symbol.name, symbol.display_path(), kind, format!("{}.html#L{}", def.path, def.line + 1), id]));
        }
        for file in self.site.files.iter().filter(|f| f.text.is_some()) {
            entries.push(json!([file.name(), file.path, "File", format!("{}.html", file.path)]));
        }
        let json = serde_json::to_string(&entries)?;
        let path = self.site.output.join(ASSET_DIR).join("search.js");
        fs::write(&path, format!("rmxSearchIndex({json});\n"))
            .with_context(|| format!("failed to write {}", path.display()))
    }
}

/// The listing page of a directory, relative to the output root.
fn dir_index(dir: &str) -> String {
    if dir.is_empty() { "index.html".to_string() } else { format!("{dir}/index.html") }
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn markdown_to_html(md: &str) -> String {
    let mut options = rmx::comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    rmx::comrak::markdown_to_html(md, &options)
}

/// The default output directory for a workspace.
pub fn default_output(metadata: &Metadata) -> PathBuf {
    metadata.target_directory.join("rmxbrowse")
}
