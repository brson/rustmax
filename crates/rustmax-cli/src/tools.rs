use rmx::{
    clap,
    serde,
};

#[derive(clap::ValueEnum)]
#[derive(clap::Subcommand)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(enum_iterator::Sequence)]
#[derive(Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Tool {
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

    /* non-rust */
    Mold,
}

pub struct ToolAttrs {
    pub display_name: &'static str,
}

