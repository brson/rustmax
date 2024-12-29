#![allow(unused)]

mod tools;
mod impls;

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
    InstallTools(CliCmdInstallTools),
    InstallTool(CliCmdInstallTool),
    ListDocs(CliCmdListDocs),
    OpenDoc(CliCmdOpenDoc),
    WriteFmtConfig(CliCmdWriteFmtConfig),
    WriteCargoDenyConfig(CliCmdWriteCargoDenyConfig),
    NewProject(CliCmdNewProject),
    RunAllChecks(CliCmdRunAllChecks),
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
struct CliCmdWriteCargoDenyConfig {
}

#[derive(clap::Args)]
struct CliCmdNewProject {
}

#[derive(clap::Args)]
struct CliCmdRunAllChecks {
}

impl CliOpts {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            CliCmd::ListTools(cmd) => cmd.run(),

            CliCmd::WriteFmtConfig(cmd) => cmd.run(),

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

impl CliCmdWriteFmtConfig {
    fn run(&self) -> AnyResult<()> {
        // todo --force
        let contents = include_bytes!("../../../rustfmt.toml");
        rmx::std::fs::write("rustfmt.toml", contents)?;
        Ok(())
    }
}

impl CliCmdWriteCargoDenyConfig {
    fn run(&self) -> AnyResult<()> {
        // todo --force
        let contents = include_bytes!("../../../configs/deny.toml");
        rmx::std::fs::write("rustfmt.toml", contents)?;
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
