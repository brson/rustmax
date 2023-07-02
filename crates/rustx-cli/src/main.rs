#![allow(unused)]

mod impls;

use rx::prelude::*;
use rx::{
    clap,
    serde,
};

fn main() -> AnyResult<()> {
    let opts = <CliOpts as clap::Parser>::parse();
    opts.run()
}

#[derive(clap::ValueEnum)]
#[derive(clap::Subcommand)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(enum_iterator::Sequence)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone)]
enum Tool {
    /* rustup itself */

    Rustup,

    /* rustup proxies */

    Cargo,
    CargoClippy,
    CargoFmt,
    CargoMiri,
    Rustc,
    Rustdoc,
    Rustfmt,
    RustGdbGui,
    RustGdb,
    RustLldb,

    /* other tools from rustup components */

    RustAnalyzer,
    Miri,
    Clippy,
    LlvmTools,
    LlvmCov,
    
    /* cargo plugins */

    CargoAudit,
    CargoBenchcmp,
    CargoCleanAll,
    CargoDeny,
    CargoDeps,
    CargoEdit,
    CargoExpand,
    CargoFuzz,
    CargoGeiger,
    CargoGenerate,
    CargoLlvmLines,
    CargoOutdated,
    CargoUdeps,
    CargoTree,
    CargoWatch,
    CargoWorkspace,
    CargoSemver, // rust-semversemver

    /* non-plugins */
    
    BasicHttpServer,
    Eva,
    Chit,
    Critcmp,
    DuDust,
    FdFind,
    Gist,
    Hexyl,
    Hyperfine,
    Jsonxf,
    Just,
    Mdbook,
    Parol,
    Ripgrep,
    Sd,
    Tokei,
    WasmOpt,
    WasmPack,
    WasmTools,
    Xsv,
}

#[derive(clap::Parser)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CliCmd,
}

#[derive(clap::Subcommand)]
enum CliCmd {
    ListTools(CliCmdListTools),
    InstallTools(CliCmdInstallTools),
    InstallTool(CliCmdInstallTool),
}

#[derive(clap::Args)]
struct CliCmdListTools {
}

#[derive(clap::Args)]
struct CliCmdInstallTools {
    tools: Vec<Tool>,
}

#[derive(clap::Args)]
struct CliCmdInstallTool {
    tool: Tool,
}

impl CliOpts {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            CliCmd::ListTools(cmd) => cmd.run(),
            _ => todo!(),
        }
    }
}

impl CliCmdListTools {
    fn run(&self) -> AnyResult<()> {
        for tool in enum_iterator::all::<Tool>() {
            todo!()
        }

        Ok(())
    }
}
