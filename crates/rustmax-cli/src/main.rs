#![allow(unused)]

mod books;
mod impls;
mod library_gen;
mod moldman;
mod rmxbook;
mod tools;

use include_dir::{include_dir, Dir};
use rmx::prelude::*;
use rmx::{clap, serde};
use std::path::Path;

use tools::Tool;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../template");

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
    /// Generate library.md with local book links (off by default during development)
    #[arg(long)]
    generate_library_page: bool,
}

#[derive(clap::Args)]
struct CliCmdRefreshLibrary {
    book: Option<String>,
}


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

            CliCmd::Rmxbook(cmd) => cmd.run(),

            CliCmd::Doctest(cmd) => cmd.run(),

            CliCmd::NewProject(cmd) => cmd.run(),

            CliCmd::WriteFmtConfig(cmd) => cmd.run(),
            CliCmd::WriteCargoDenyConfig(cmd) => cmd.run(),
            CliCmd::WriteClippyControlConfig(cmd) => cmd.run(),

            CliCmd::RunAllChecks(cmd) => cmd.run(),

            _ => todo!(),
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
