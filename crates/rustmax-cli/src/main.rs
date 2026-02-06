#![allow(unused)]

mod books;
mod impls;
mod library_gen;
mod moldman;
mod rmxbook;
mod tools;
mod topics;

use include_dir::{include_dir, Dir};
use rmx::prelude::*;
use rmx::{clap, serde, serde_json};
use std::path::Path;

use tools::Tool;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/template");

fn main() -> AnyResult<()> {
    let opts = <CliOpts as clap::Parser>::parse();
    opts.run()
}

#[derive(clap::Parser)]
#[command(version)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CliCmd,
}

#[derive(clap::Subcommand)]
enum CliCmd {
    /// List all available tools.
    ListTools(CliCmdListTools),

    /// Install all tools.
    InstallTools(CliCmdInstallTools),
    /// Update all tools.
    UpdateTools(CliCmdUpdateTools),
    /// Uninstall all tools.
    UninstallTools(CliCmdUninstallTools),
    /// Show status of all tools.
    ToolsStatus(CliCmdToolsStatus),

    /// Install a specific tool.
    InstallTool(CliCmdInstallTool),
    /// Update a specific tool.
    UpdateTool(CliCmdUpdateTool),
    /// Uninstall a specific tool.
    UninstallTool(CliCmdUninstallTool),
    /// Show status of a specific tool.
    ToolStatus(CliCmdToolStatus),

    /// List books in the library.
    ListLibrary(CliCmdListLibrary),
    /// Build the library or a specific book.
    BuildLibrary(CliCmdBuildLibrary),
    /// Refresh the library or a specific book.
    RefreshLibrary(CliCmdRefreshLibrary),
    /// Generate the library.html page.
    GenerateLibraryPage(CliCmdGenerateLibraryPage),

    /// Build a book using the rmxbook renderer.
    Rmxbook(CliCmdRmxbook),

    /// Run doctests from crate examples.
    Doctest(CliCmdDoctest),

    /// Create a new project from template.
    NewProject(CliCmdNewProject),

    /// Write rustfmt.toml configuration file.
    WriteFmtConfig(CliCmdWriteFmtConfig),
    /// Write deny.toml configuration file.
    WriteCargoDenyConfig(CliCmdWriteCargoDenyConfig),
    /// Write clippy-control.toml configuration file.
    WriteClippyControlConfig(CliCmdWriteClippyControlConfig),

    /// Run all code quality checks.
    RunAllChecks(CliCmdRunAllChecks),

    /// Generate documentation from rustdoc JSON.
    Rustdoc(CliCmdRustdoc),

    /// Validate the topic index.
    ValidateTopics(CliCmdValidateTopics),

    /// Summarize the topic index.
    SummarizeTopics(CliCmdSummarizeTopics),

    /// Export topic search index as JSON for client-side search.
    ExportSearchIndex(CliCmdExportSearchIndex),

    /// Search the topic index from the command line.
    Search(CliCmdSearch),
}

#[derive(clap::Args)]
struct CliCmdListTools {}

#[derive(clap::Args)]
struct CliCmdInstallTools {}

#[derive(clap::Args)]
struct CliCmdUpdateTools {}

#[derive(clap::Args)]
struct CliCmdUninstallTools {}

#[derive(clap::Args)]
struct CliCmdToolsStatus {}

#[derive(clap::Args)]
struct CliCmdInstallTool {
    tool: Tool,
}

#[derive(clap::Args)]
struct CliCmdUpdateTool {
    tool: Tool,
}

#[derive(clap::Args)]
struct CliCmdUninstallTool {
    tool: Tool,
}

#[derive(clap::Args)]
struct CliCmdToolStatus {
    tool: Tool,
}

#[derive(clap::Args)]
struct CliCmdListLibrary {}

#[derive(clap::Args)]
struct CliCmdBuildLibrary {
    book: Option<String>,
    /// Force git clone/fetch operations (by default skipped for dev speed)
    #[arg(long)]
    fetch: bool,
    /// Generate library.html with local book links (off by default during development)
    #[arg(long)]
    generate_library_page: bool,
}

#[derive(clap::Args)]
struct CliCmdRefreshLibrary {
    book: Option<String>,
}

#[derive(clap::Args)]
struct CliCmdGenerateLibraryPage {}


#[derive(clap::Args)]
struct CliCmdRmxbook {
    /// Input directory containing book.toml and src/
    input: String,
    /// Output directory for rendered HTML
    output: String,
}

#[derive(clap::Args)]
struct CliCmdDoctest {
    /// Test name filter (substring match).
    filter: Option<String>,

    /// Number of test threads.
    #[arg(long)]
    test_threads: Option<usize>,

    /// Don't capture test output.
    #[arg(long)]
    nocapture: bool,

    /// Force rebuild of test crate.
    #[arg(long)]
    rebuild: bool,

    /// Run ignored tests.
    #[arg(long)]
    ignored: bool,
}

#[derive(clap::Args)]
struct CliCmdNewProject {
    /// Project name (will prompt if not provided).
    name: Option<String>,
}

#[derive(clap::Args)]
struct CliCmdWriteFmtConfig {}

#[derive(clap::Args)]
struct CliCmdWriteCargoDenyConfig {}

#[derive(clap::Args)]
struct CliCmdWriteClippyControlConfig {}

#[derive(clap::Args)]
struct CliCmdRunAllChecks {}

#[derive(clap::Args)]
struct CliCmdValidateTopics {
    /// Path to the topics directory.
    #[arg(default_value = "src/topics")]
    path: String,
}

#[derive(clap::Args)]
struct CliCmdSummarizeTopics {
    /// Path to the topics directory.
    #[arg(default_value = "src/topics")]
    path: String,

    /// Show full topic lists (not just counts).
    #[arg(long, short)]
    verbose: bool,
}

#[derive(clap::Args)]
struct CliCmdExportSearchIndex {
    /// Path to the topics directory.
    #[arg(default_value = "src/topics")]
    path: String,

    /// Output JSON file path.
    #[arg(short, long, default_value = "work/search-index.json")]
    output: String,
}

#[derive(clap::Args)]
struct CliCmdSearch {
    /// Search query.
    query: Vec<String>,

    /// Path to search-index.json.
    #[arg(short, long, default_value = "work/search-index.json")]
    index: String,

    /// Path to the search-cli.js script.
    #[arg(long, default_value = "www/search-cli.js")]
    script: String,
}

#[derive(clap::Args)]
struct CliCmdRustdoc {
    #[command(subcommand)]
    action: RustdocAction,
}

#[derive(clap::Subcommand)]
enum RustdocAction {
    /// Build docs from rustdoc JSON file or directory.
    Build {
        /// Path to rustdoc JSON file or directory containing JSON files.
        ///
        /// If a file, builds docs for a single crate.
        /// If a directory, builds docs for all crates with cross-crate linking.
        json_path: std::path::PathBuf,
        /// Output directory.
        #[arg(short, long, default_value = "target/rmxdoc")]
        output: std::path::PathBuf,
        /// Include private items.
        #[arg(long)]
        document_private_items: bool,
    },
}

impl CliOpts {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            CliCmd::ListTools(cmd) => cmd.run(),

            CliCmd::InstallTools(cmd) => cmd.run(),
            CliCmd::UpdateTools(cmd) => cmd.run(),
            CliCmd::UninstallTools(cmd) => cmd.run(),
            CliCmd::ToolsStatus(cmd) => cmd.run(),

            CliCmd::InstallTool(cmd) => cmd.run(),
            CliCmd::UpdateTool(cmd) => cmd.run(),
            CliCmd::UninstallTool(cmd) => cmd.run(),
            CliCmd::ToolStatus(cmd) => cmd.run(),

            CliCmd::ListLibrary(cmd) => cmd.run(),
            CliCmd::BuildLibrary(cmd) => cmd.run(),
            CliCmd::RefreshLibrary(cmd) => cmd.run(),
            CliCmd::GenerateLibraryPage(cmd) => cmd.run(),

            CliCmd::Rmxbook(cmd) => cmd.run(),

            CliCmd::Doctest(cmd) => cmd.run(),

            CliCmd::NewProject(cmd) => cmd.run(),

            CliCmd::WriteFmtConfig(cmd) => cmd.run(),
            CliCmd::WriteCargoDenyConfig(cmd) => cmd.run(),
            CliCmd::WriteClippyControlConfig(cmd) => cmd.run(),

            CliCmd::RunAllChecks(cmd) => cmd.run(),
            CliCmd::Rustdoc(cmd) => cmd.run(),
            CliCmd::ValidateTopics(cmd) => cmd.run(),
            CliCmd::SummarizeTopics(cmd) => cmd.run(),
            CliCmd::ExportSearchIndex(cmd) => cmd.run(),
            CliCmd::Search(cmd) => cmd.run(),
        }
    }
}

impl CliCmdListTools {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            if !tool.attrs().impl_complete {
                continue;
            }
            println!("{}", tool.attrs().display_name);
        }

        Ok(())
    }
}

impl CliCmdInstallTools {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            if !tool.attrs().impl_complete {
                continue;
            }
            tool.install()?;
        }
        Ok(())
    }
}

impl CliCmdUpdateTools {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            if !tool.attrs().impl_complete {
                continue;
            }
            tool.update()?;
        }
        Ok(())
    }
}

impl CliCmdUninstallTools {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            if !tool.attrs().impl_complete {
                continue;
            }
            tool.uninstall()?;
        }
        Ok(())
    }
}

impl CliCmdToolsStatus {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            if !tool.attrs().impl_complete {
                continue;
            }
            tool.status()?;
        }
        Ok(())
    }
}

impl CliCmdInstallTool {
    fn run(&self) -> AnyResult<()> {
        self.tool.install()
    }
}

impl CliCmdUpdateTool {
    fn run(&self) -> AnyResult<()> {
        self.tool.update()
    }
}

impl CliCmdUninstallTool {
    fn run(&self) -> AnyResult<()> {
        self.tool.uninstall()
    }
}

impl CliCmdToolStatus {
    fn run(&self) -> AnyResult<()> {
        self.tool.status()
    }
}

impl CliCmdListLibrary {
    fn run(&self) -> AnyResult<()> {
        let root = &Path::new(".");
        books::list_library(root)
    }
}

impl CliCmdBuildLibrary {
    fn run(&self) -> AnyResult<()> {
        let root = &Path::new(".");
        let no_fetch = !self.fetch; // Invert the logic: fetch=false means no_fetch=true
        match self.book {
            None => books::build_library(root, no_fetch, self.generate_library_page),
            Some(ref book) => books::build_one_book(root, book, no_fetch),
        }
    }
}

impl CliCmdRefreshLibrary {
    fn run(&self) -> AnyResult<()> {
        let root = &Path::new(".");
        match self.book {
            None => books::refresh_library(root),
            Some(ref book) => books::refresh_one_book(root, book),
        }
    }
}

impl CliCmdGenerateLibraryPage {
    fn run(&self) -> AnyResult<()> {
        library_gen::generate_library_page()
    }
}

impl CliCmdRmxbook {
    fn run(&self) -> AnyResult<()> {
        let input = Path::new(&self.input);
        let output = Path::new(&self.output);
        println!("Building {} -> {}", input.display(), output.display());
        rmxbook::build(input, output)?;
        println!("Done");
        Ok(())
    }
}

impl CliCmdDoctest {
    fn run(&self) -> AnyResult<()> {
        let doc_dir = Path::new("crates/rustmax/doc-src");
        let work_dir = Path::new("work/doctest");

        let mut test_args = vec![];
        if let Some(filter) = &self.filter {
            test_args.push(filter.clone());
        }
        if let Some(threads) = self.test_threads {
            test_args.push(format!("--test-threads={}", threads));
        }
        if self.nocapture {
            test_args.push("--nocapture".to_string());
        }
        if self.ignored {
            test_args.push("--ignored".to_string());
        }

        rustmax_doctest::run_doctests(doc_dir, work_dir, &test_args, self.rebuild)
    }
}

impl CliCmdNewProject {
    fn run(&self) -> AnyResult<()> {
        let temp_dir = rmx::tempfile::tempdir()?;

        extract_dir(&TEMPLATE_DIR, temp_dir.path())?;

        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("generate")
            .arg("--path")
            .arg(temp_dir.path());

        if let Some(name) = &self.name {
            cmd.arg("--name").arg(name);
        }

        let status = cmd.status()?;
        if !status.success() {
            rmx::anyhow::bail!("cargo generate failed");
        }

        Ok(())
    }
}

fn extract_dir(dir: &Dir, dest: &Path) -> AnyResult<()> {
    for file in dir.files() {
        let path = dest.join(file.path());
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, file.contents())?;
    }
    for subdir in dir.dirs() {
        extract_dir(subdir, dest)?;
    }
    Ok(())
}

impl CliCmdWriteFmtConfig {
    fn run(&self) -> AnyResult<()> {
        // todo --force
        let contents = include_bytes!("../assets/rustfmt.toml");
        rmx::std::fs::write("rustfmt.toml", contents)?;
        Ok(())
    }
}

impl CliCmdWriteCargoDenyConfig {
    fn run(&self) -> AnyResult<()> {
        // todo --force
        let contents = include_bytes!("../assets/deny.toml");
        rmx::std::fs::write("deny.toml", contents)?;
        Ok(())
    }
}

impl CliCmdWriteClippyControlConfig {
    fn run(&self) -> AnyResult<()> {
        // todo --force
        let contents = include_bytes!("../assets/clippy-control.toml");
        rmx::std::fs::write("clippy-control.toml", contents)?;
        Ok(())
    }
}

impl CliCmdRunAllChecks {
    fn run(&self) -> AnyResult<()> {
        // cargo-clippy-control
        // cargo-fmt
        // cargo-deny
        // cargo-audit
        todo!()
    }
}

impl CliCmdValidateTopics {
    fn run(&self) -> AnyResult<()> {
        let path = Path::new(&self.path);
        println!("Loading topics from {}...", path.display());

        let index = topics::TopicIndex::load(path)?;
        let result = index.validate();
        result.print_report();

        if result.is_ok() {
            Ok(())
        } else {
            bail!("topic index validation failed")
        }
    }
}

impl CliCmdSummarizeTopics {
    fn run(&self) -> AnyResult<()> {
        let path = Path::new(&self.path);
        let index = topics::TopicIndex::load(path)?;
        index.print_summary(self.verbose);
        Ok(())
    }
}

impl CliCmdExportSearchIndex {
    fn run(&self) -> AnyResult<()> {
        let path = Path::new(&self.path);
        println!("Loading topics from {}...", path.display());

        let index = topics::TopicIndex::load(path)?;
        let result = index.validate();

        if !result.is_ok() {
            result.print_report();
            bail!("topic index validation failed");
        }

        let entries = index.export_search_index();
        println!("Generated {} search entries.", entries.len());

        // Ensure output directory exists.
        let output_path = Path::new(&self.output);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write JSON.
        let json = serde_json::to_string_pretty(&entries)?;
        std::fs::write(output_path, json)?;

        println!("Wrote search index to {}", output_path.display());
        Ok(())
    }
}

/// A single search result deserialized from the Node.js search output.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsSearchResult {
    entry: JsSearchEntry,
    score: f64,
    matched_text: Option<String>,
    match_type: String,
}

#[derive(serde::Deserialize)]
struct JsSearchEntry {
    id: String,
    name: String,
    category: String,
    brief: String,
    path: Option<String>,
}

/// A search result for TOML serialization.
#[derive(serde::Serialize)]
struct SearchResult {
    name: String,
    category: String,
    brief: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    score: f64,
    match_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_info: Option<String>,
}

#[derive(serde::Serialize)]
struct SearchOutput {
    results: Vec<SearchResult>,
}

impl CliCmdSearch {
    fn run(&self) -> AnyResult<()> {
        let query = self.query.join(" ");
        if query.is_empty() {
            bail!("search query is required");
        }

        let index_path = Path::new(&self.index);
        if !index_path.exists() {
            bail!(
                "search index not found at {}; run `rustmax export-search-index` first",
                index_path.display()
            );
        }

        let script_path = Path::new(&self.script);
        if !script_path.exists() {
            bail!("search script not found at {}", script_path.display());
        }

        // Spawn node to run the search.
        let output = std::process::Command::new("node")
            .arg(script_path)
            .arg(index_path)
            .arg(&query)
            .output()
            .context("failed to run node; is Node.js installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("node search failed: {}", stderr.trim());
        }

        let stdout = String::from_utf8(output.stdout)
            .context("node output was not valid utf-8")?;

        let js_results: Vec<JsSearchResult> = serde_json::from_str(&stdout)
            .context("failed to parse search results from node")?;

        if js_results.is_empty() {
            println!("No results for \"{}\".", query);
            return Ok(());
        }

        // Convert to TOML-friendly output.
        let results: Vec<SearchResult> = js_results
            .into_iter()
            .map(|r| {
                let match_info = r.matched_text.map(|t| format!("aka \"{}\"", t));
                SearchResult {
                    name: r.entry.name,
                    category: r.entry.category,
                    brief: r.entry.brief,
                    path: r.entry.path,
                    score: r.score,
                    match_type: r.match_type,
                    match_info,
                }
            })
            .collect();

        let output = SearchOutput { results };
        let toml_str = toml::to_string_pretty(&output)
            .context("failed to serialize results as toml")?;

        print!("{}", toml_str);

        Ok(())
    }
}

impl CliCmdRustdoc {
    fn run(&self) -> AnyResult<()> {
        match &self.action {
            RustdocAction::Build {
                json_path,
                output,
                document_private_items,
            } => {
                if json_path.is_dir() {
                    // Directory mode: load all JSON files and render with cross-crate linking.
                    println!("Loading rustdoc JSON files from {}...", json_path.display());

                    let doc_set = rustmax_rustdoc::RustDocSet::from_json_dir(json_path)?
                        .output_dir(output.clone())
                        .include_private(*document_private_items);

                    println!("Found {} crates.", doc_set.crates.len());
                    println!("Rendering to {}...", output.display());
                    doc_set.render()?;

                    println!("Done. Open {}/index.html to view.", output.display());
                } else {
                    // Single file mode: load one JSON file.
                    println!("Loading rustdoc JSON from {}...", json_path.display());

                    let doc = rustmax_rustdoc::RustDoc::from_json(json_path)?
                        .output_dir(output.clone())
                        .include_private(*document_private_items);

                    println!("Rendering to {}...", output.display());
                    doc.render()?;

                    println!("Done. Open {}/index.html to view.", output.display());
                }
                Ok(())
            }
        }
    }
}
