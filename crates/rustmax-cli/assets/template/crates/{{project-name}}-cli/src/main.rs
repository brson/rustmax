#![allow(unused)]

use rmx::prelude::*;

use rmx::clap::{self, Parser as _};
use rmx::std::path::PathBuf;

fn main() -> AnyResult<()> {
    rmx::extras::init_crate_name(env!("CARGO_CRATE_NAME"));

    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
    #[command(flatten)]
    args: Args,
}

#[derive(clap::Subcommand)]
enum Command {
    Run(RunCommand),
}

#[derive(clap::Args)]
struct Args {
    #[arg(default_value = "config.toml")]
    config_path: PathBuf,
}

#[derive(clap::Args)]
struct RunCommand {
}

impl Cli {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            Command::Run(cmd) => cmd.run(&self.args),
        }
    }
}

impl RunCommand {
    fn run(&self, _args: &Args) -> AnyResult<()> {
        info!("hello world");

        Ok(())
    }
}
