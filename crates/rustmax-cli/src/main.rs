#![allow(unused)]

mod tools;
mod impls;
mod moldman;
mod books;

use rmx::prelude::*;
use rmx::{
    clap,
    serde,
};

use tools::Tool;


fn main() -> AnyResult<()> {
    let opts = <CliOpts as clap::Parser>::parse();
    opts.run()
}

#[derive(clap::Parser)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CliCmd,
}

#[derive(clap::Subcommand)]
enum CliCmd {
    ListTools(CliCmdListTools),
    InstallTools,
    InstallTool(CliCmdInstallTool),
    UpdateTools,
    UpdateTool(CliCmdUpdateTool),
    UninstallTools,
    UninstallTool(CliCmdUninstallTool),
    ToolsStatus,
    ToolStatus(CliCmdToolStatus),

    ListLibrary(CliCmdListLibrary),
    BuildLibrary(CliCmdBuildLibrary),

    NewProject,

    WriteFmtConfig(CliCmdWriteFmtConfig),
    WriteCargoDenyConfig(CliCmdWriteCargoDenyConfig),
    WriteClippyControlConfig(CliCmdWriteClippyControlConfig),

    RunAllChecks(CliCmdRunAllChecks),
}

#[derive(clap::Args)]
struct CliCmdListTools {
}

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
struct CliCmdListLibrary {
}

#[derive(clap::Args)]
struct CliCmdBuildLibrary {
    book: Option<String>,
}

#[derive(clap::Args)]
struct CliCmdWriteFmtConfig {
}

#[derive(clap::Args)]
struct CliCmdWriteCargoDenyConfig {
}

#[derive(clap::Args)]
struct CliCmdWriteClippyControlConfig {
}

#[derive(clap::Args)]
struct CliCmdRunAllChecks {
}

impl CliOpts {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            CliCmd::ListTools(cmd) => cmd.run(),
            CliCmd::InstallTool(cmd) => cmd.run(),
            CliCmd::UpdateTool(cmd) => cmd.run(),
            CliCmd::UninstallTool(cmd) => cmd.run(),
            CliCmd::ToolStatus(cmd) => cmd.run(),

            CliCmd::ListLibrary(cmd) => cmd.run(),
            CliCmd::BuildLibrary(cmd) => cmd.run(),

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
            println!("{}", tool.attrs().display_name);
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
        books::list_library()
    }
}

impl CliCmdBuildLibrary {
    fn run(&self) -> AnyResult<()> {
        match self.book {
            None => books::build_library(),
            Some(ref book) => books::build_one_book(book),
        }
    }
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
