use rx::prelude::*;
use rx::clap::{self, Parser as _};
use std::path::PathBuf;

fn main() -> AnyResult<()> {
    rx::extras::init();

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
    Check(CheckCommand),
}

#[derive(clap::Args)]
struct Args {
    #[arg(default_value = "clippy-control.toml")]
    config_path: PathBuf,
}

#[derive(clap::Args)]
struct CheckCommand {
}

impl Cli {
    fn run(&self) -> AnyResult<()> {
        match &self.cmd {
            Command::Check(cmd) => cmd.run(&self.args),
        }
    }
}

impl CheckCommand {
    fn run(&self, _args: &Args) -> AnyResult<()> {
        info!("hello world");

        Ok(())
    }
}
