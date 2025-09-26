// CLI interface module.

pub mod commands;
pub mod repl;

use rmx::prelude::*;

// Build the CLI application.
pub fn build_cli() -> rmx::clap::Command {
    use rmx::clap::{Arg, Command};

    Command::new(crate::NAME)
        .version(crate::VERSION)
        .about("Rustmax Suite - Developer Tools Hub")
        .long_about(
            "A comprehensive developer tools hub that integrates all rustmax crates \
             through practical utilities and workflows."
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path (TOML or JSON5)")
                .global(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(rmx::clap::ArgAction::SetTrue)
                .global(true)
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress non-error output")
                .action(rmx::clap::ArgAction::SetTrue)
                .global(true)
        )
        .subcommands(commands::all_commands())
}

// Main entry point for CLI.
pub async fn run(args: Vec<String>) -> crate::Result<()> {
    let app = build_cli();
    let matches = app.try_get_matches_from(args)?;

    // Load configuration.
    let mut config = if let Some(config_path) = matches.get_one::<String>("config") {
        crate::infrastructure::config::Config::load(config_path)?
    } else {
        crate::infrastructure::config::Config::default()
    };

    // Apply command-line overrides.
    if matches.get_flag("verbose") {
        config.cli.verbose = true;
        config.log_level = "debug".to_string();
    }
    if matches.get_flag("quiet") {
        config.log_level = "error".to_string();
    }

    // Merge environment variables.
    config.merge_env()?;

    // Initialize logging.
    crate::infrastructure::logging::init_logging(&config.log_level)?;

    // Create application state.
    let state = crate::infrastructure::state::AppState::new(config.clone());

    // Handle subcommands.
    match matches.subcommand() {
        Some((name, sub_matches)) => {
            commands::handle_command(name, sub_matches, &state).await
        }
        None => {
            if config.cli.interactive {
                // Enter REPL mode.
                repl::run_repl(&state).await
            } else {
                // Show help.
                println!("{}", build_cli().render_help());
                Ok(())
            }
        }
    }
}