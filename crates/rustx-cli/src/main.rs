mod impls;

use rustx::{
    clap,
    serde,
};

fn main() {
    let _opts = <CliOpts as clap::Parser>::parse();
}

#[derive(clap::ValueEnum)]
#[derive(clap::Subcommand)]
#[derive(serde::Serialize, serde::Deserialize)]
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
