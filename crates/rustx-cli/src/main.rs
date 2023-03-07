fn main() {
    let _opts = <CliOpts as clap::Parser>::parse();
}

#[derive(clap::ValueEnum)]
#[derive(clap::Subcommand)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone)]
enum CargoPlugins {
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
}

#[derive(clap::ValueEnum)]
#[derive(clap::Subcommand)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Clone)]
enum CargoExes {
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
    WasmTools,
    Xsv,
}

#[derive(clap::Subcommand)]
#[derive(Clone)]
enum Tool {
    #[command(flatten)]
    Plugin(CargoPlugins),
    #[command(flatten)]
    Exe(CargoExes),
}

#[derive(clap::Parser)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CliCmd,
}

#[derive(clap::Subcommand)]
enum CliCmd {
    InstallTools(CliCmdInstallTools),
    INstallTool(CliCmdInstallTool),
}

#[derive(clap::Args)]
struct CliCmdInstallTools {
    #[command(subcommand)]
    tools: Tool,
}

#[derive(clap::Args)]
struct CliCmdInstallTool {
    #[command(subcommand)]
    tool: Tool,
}
