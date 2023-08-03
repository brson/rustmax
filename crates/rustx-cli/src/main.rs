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
#[derive(Clone)]
#[serde(rename_all = "kebab-case")]
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
    CargoHack,
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

struct ToolAttrs {
    display_name: &'static str,
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
    ListDocs(CliCmdListDocs),
    OpenDoc(CliCmdOpenDoc),
    WriteFmtConfig(CliCmdWriteFmtConfig),
    WriteClippyConfig(CliCmdWriteClippyConfig),
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

#[derive(clap::Args)]
struct CliCmdListDocs {
}

#[derive(clap::Args)]
struct CliCmdOpenDoc {
}

#[derive(clap::Args)]
struct CliCmdWriteFmtConfig {
}

#[derive(clap::Args)]
struct CliCmdWriteClippyConfig {
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
            println!("{}", tool.attrs().display_name);
        }

        Ok(())
    }
}
