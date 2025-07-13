use rmx::{clap, serde};

#[derive(
    clap::ValueEnum,
    clap::Subcommand,
    serde::Serialize,
    serde::Deserialize,
    enum_iterator::Sequence,
    Clone,
)]
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
    CargoCleanAll,
    CargoDeny,
    CargoEdit,
    CargoGenerate,
    CargoOutdated,
    CargoTree,

    /* non-plugin cargo programs */
    BasicHttpServer,
    DuDust,
    FdFind,
    Gist,
    Jsonxf,
    Jaq,
    Just,
    Mdbook,
    Ripgrep,
    Sd,
    Tokei,

    /* non-rust */
    Mold,
}

pub struct ToolAttrs {
    pub display_name: &'static str,
    // Temporary status indicating whether all rustmax commands are implemented
    pub impl_complete: bool,
}
