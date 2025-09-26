// Interactive REPL for the CLI.

use rmx::prelude::*;
use crate::infrastructure::state::AppState;

// Run the interactive REPL.
pub async fn run_repl(state: &AppState) -> crate::Result<()> {
    use rmx::rustyline::DefaultEditor;

    println!("Rustmax Suite REPL v{}", crate::VERSION);
    println!("Type 'help' for commands, 'exit' to quit");

    let config = state.config().await;
    let history_file = config.cli.history_file.as_ref().map(|p| p.to_string_lossy().to_string());

    // Create readline editor.
    let mut rl = DefaultEditor::new()?;

    // Load history if available.
    if let Some(ref history_file) = history_file {
        let _ = rl.load_history(history_file);
    }

    loop {
        match rl.readline("rmx> ") {
            Ok(line) => {
                let line = line.trim();

                // Add to history.
                let _ = rl.add_history_entry(line);

                // Handle special REPL commands.
                match line {
                    "exit" | "quit" => break,
                    "help" => {
                        print_repl_help();
                        continue;
                    }
                    "clear" => {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }

                // Parse and execute command.
                let args = match shlex::split(line) {
                    Some(args) => args,
                    None => {
                        eprintln!("Error: Invalid command syntax");
                        continue;
                    }
                };

                // Prepend program name for clap parsing.
                let mut full_args = vec!["rmx-suite".to_string()];
                full_args.extend(args);

                // Try to execute command.
                match execute_repl_command(full_args, state).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(rmx::rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(rmx::rustyline::error::ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history.
    if let Some(ref history_file) = history_file {
        let _ = rl.save_history(history_file);
    }

    println!("Goodbye!");
    Ok(())
}

// Execute a command in REPL context.
async fn execute_repl_command(args: Vec<String>, state: &AppState) -> crate::Result<()> {
    let app = crate::cli::build_cli();

    match app.try_get_matches_from(args) {
        Ok(matches) => {
            // Handle subcommands.
            match matches.subcommand() {
                Some((name, sub_matches)) => {
                    crate::cli::commands::handle_command(name, sub_matches, state).await
                }
                None => {
                    println!("Type 'help' for available commands");
                    Ok(())
                }
            }
        }
        Err(e) => {
            // Don't exit on error in REPL, just show the error.
            eprintln!("{}", e);
            Ok(())
        }
    }
}

// Print REPL help.
fn print_repl_help() {
    println!("\n=== Rustmax Suite REPL Commands ===\n");
    println!("Available commands:");
    println!("  scan [path]         - Scan project directory");
    println!("  analyze [path]      - Analyze dependencies");
    println!("  test [pattern]      - Run tests");
    println!("  build               - Build project");
    println!("  format [path]       - Format code");
    println!("  metrics             - Display metrics");
    println!("  server              - Start web server");
    println!();
    println!("Legacy commands:");
    println!("  greet [name]        - Greeting demo");
    println!("  count [n]           - Counting demo");
    println!("  math <a> <b>        - Math operations");
    println!("  file [content]      - File operations");
    println!("  serialize [format]  - Serialization demo");
    println!("  crypto [algo]       - Crypto demo");
    println!("  time [lib]          - Time demo");
    println!("  regex [pattern]     - Regex demo");
    println!("  async [type]        - Async demo");
    println!("  parallel [items]    - Parallel demo");
    println!();
    println!("REPL commands:");
    println!("  help                - Show this help");
    println!("  clear               - Clear screen");
    println!("  exit/quit           - Exit REPL");
    println!();
}

// Helper for parsing shell-like command lines.
mod shlex {
    pub fn split(s: &str) -> Option<Vec<String>> {
        let mut args = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;

        for c in s.chars() {
            if escape_next {
                current.push(c);
                escape_next = false;
                continue;
            }

            match c {
                '\\' => escape_next = true,
                '"' => in_quotes = !in_quotes,
                ' ' | '\t' if !in_quotes => {
                    if !current.is_empty() {
                        args.push(current);
                        current = String::new();
                    }
                }
                _ => current.push(c),
            }
        }

        if in_quotes {
            return None; // Unclosed quote.
        }

        if !current.is_empty() {
            args.push(current);
        }

        Some(args)
    }
}