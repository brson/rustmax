fn main() {
    println!("Hello, world!");
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(clap::ValueEnum)]
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
    RustSemverver,
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(clap::ValueEnum)]
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

#[derive(clap::Parser)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CliCmd,
}

#[derive(clap::Subcommand)]
enum CliCmd {
    InstallTools(CliCmdInstallTools),
}

#[derive(clap::Args)]
struct CliCmdInstallTools {
}
