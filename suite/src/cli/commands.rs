// CLI command definitions and handlers.

use rmx::prelude::*;
use crate::infrastructure::state::AppState;

// Get all CLI commands.
pub fn all_commands() -> Vec<rmx::clap::Command> {
    use rmx::clap::{Arg, Command};

    vec![
        // Core commands.
        Command::new("scan")
            .about("Scan project directory for analysis")
            .arg(
                Arg::new("path")
                    .help("Directory path to scan")
                    .default_value(".")
                    .index(1)
            )
            .arg(
                Arg::new("depth")
                    .short('d')
                    .long("depth")
                    .help("Maximum scan depth")
                    .value_parser(rmx::clap::value_parser!(usize))
            ),

        Command::new("analyze")
            .about("Analyze project dependencies")
            .arg(
                Arg::new("path")
                    .help("Path to Cargo.toml")
                    .default_value("./Cargo.toml")
                    .index(1)
            )
            .arg(
                Arg::new("check-updates")
                    .long("check-updates")
                    .help("Check for dependency updates")
                    .action(rmx::clap::ArgAction::SetTrue)
            ),

        Command::new("test")
            .about("Run project tests")
            .arg(
                Arg::new("pattern")
                    .help("Test name pattern")
                    .index(1)
            )
            .arg(
                Arg::new("parallel")
                    .short('p')
                    .long("parallel")
                    .help("Run tests in parallel")
                    .action(rmx::clap::ArgAction::SetTrue)
            ),

        Command::new("build")
            .about("Build project")
            .arg(
                Arg::new("release")
                    .short('r')
                    .long("release")
                    .help("Build in release mode")
                    .action(rmx::clap::ArgAction::SetTrue)
            )
            .arg(
                Arg::new("features")
                    .short('f')
                    .long("features")
                    .help("Space-separated feature list")
                    .value_delimiter(' ')
                    .num_args(1..)
            ),

        Command::new("format")
            .about("Format source code")
            .arg(
                Arg::new("path")
                    .help("Path to format")
                    .default_value(".")
                    .index(1)
            )
            .arg(
                Arg::new("check")
                    .long("check")
                    .help("Check formatting without modifying files")
                    .action(rmx::clap::ArgAction::SetTrue)
            ),

        Command::new("metrics")
            .about("Display application metrics")
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .help("Output format")
                    .value_parser(["json", "toml", "text"])
                    .default_value("text")
            ),

        Command::new("server")
            .about("Start web server")
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .help("Server port")
                    .value_parser(rmx::clap::value_parser!(u16))
            )
            .arg(
                Arg::new("host")
                    .short('h')
                    .long("host")
                    .help("Server host")
            ),

        Command::new("repl")
            .about("Start interactive REPL"),

        // Legacy commands for backward compatibility.
        Command::new("greet")
            .about("Greet a user")
            .arg(
                Arg::new("name")
                    .help("Name to greet")
                    .default_value("World")
                    .index(1)
            ),

        Command::new("count")
            .about("Count to a number")
            .arg(
                Arg::new("number")
                    .help("Number to count to")
                    .default_value("10")
                    .index(1)
                    .value_parser(rmx::clap::value_parser!(i32))
            ),

        Command::new("math")
            .about("Perform math operations")
            .arg(
                Arg::new("a")
                    .help("First number")
                    .index(1)
                    .required(true)
                    .value_parser(rmx::clap::value_parser!(i32))
            )
            .arg(
                Arg::new("b")
                    .help("Second number")
                    .index(2)
                    .required(true)
                    .value_parser(rmx::clap::value_parser!(i32))
            ),

        Command::new("file")
            .about("File operations demo")
            .arg(
                Arg::new("content")
                    .help("Content to write")
                    .default_value("Hello from rustmax suite!")
                    .index(1)
            ),

        Command::new("serialize")
            .about("Serialization demo")
            .arg(
                Arg::new("format")
                    .help("Serialization format")
                    .value_parser(["json", "toml"])
                    .default_value("json")
                    .index(1)
            )
            .arg(
                Arg::new("data")
                    .help("Data to serialize")
                    .default_value("default")
                    .index(2)
            ),

        Command::new("crypto")
            .about("Cryptographic operations demo")
            .arg(
                Arg::new("algorithm")
                    .help("Hash algorithm")
                    .value_parser(["blake3", "sha256"])
                    .default_value("blake3")
                    .index(1)
            )
            .arg(
                Arg::new("data")
                    .help("Data to hash")
                    .default_value("rustmax-suite")
                    .index(2)
            ),

        Command::new("time")
            .about("Date/time operations demo")
            .arg(
                Arg::new("library")
                    .help("Time library to use")
                    .value_parser(["chrono", "jiff"])
                    .default_value("chrono")
                    .index(1)
            )
            .arg(
                Arg::new("operation")
                    .help("Operation to perform")
                    .default_value("now")
                    .index(2)
            ),

        Command::new("regex")
            .about("Regex operations demo")
            .arg(
                Arg::new("pattern")
                    .help("Regex pattern")
                    .default_value(r"\b\w+\b")
                    .index(1)
            )
            .arg(
                Arg::new("text")
                    .help("Text to match")
                    .default_value("Hello world 123!")
                    .index(2)
            ),

        Command::new("async")
            .about("Async operations demo")
            .arg(
                Arg::new("operation")
                    .help("Async operation type")
                    .value_parser(["futures", "tokio"])
                    .default_value("futures")
                    .index(1)
            )
            .arg(
                Arg::new("count")
                    .help("Number of tasks")
                    .default_value("3")
                    .index(2)
                    .value_parser(rmx::clap::value_parser!(usize))
            ),

        Command::new("parallel")
            .about("Parallel processing demo")
            .arg(
                Arg::new("items")
                    .help("Number of items")
                    .default_value("100")
                    .index(1)
                    .value_parser(rmx::clap::value_parser!(usize))
            )
            .arg(
                Arg::new("threads")
                    .help("Number of threads")
                    .index(2)
                    .value_parser(rmx::clap::value_parser!(usize))
            ),
    ]
}

// Handle command execution.
pub async fn handle_command(
    name: &str,
    matches: &rmx::clap::ArgMatches,
    state: &AppState,
) -> crate::Result<()> {
    use crate::services;

    state.increment_metric(crate::infrastructure::state::MetricType::OperationTotal).await;

    match name {
        // Core commands.
        "scan" => {
            let path = matches.get_one::<String>("path").unwrap();
            let depth = matches.get_one::<usize>("depth").cloned();
            services::scanner::scan_directory(path, depth, state).await?;
        }

        "analyze" => {
            let path = matches.get_one::<String>("path").unwrap();
            let check_updates = matches.get_flag("check-updates");
            services::analyzer::analyze_dependencies(path, check_updates, state).await?;
        }

        "test" => {
            let pattern = matches.get_one::<String>("pattern");
            let parallel = matches.get_flag("parallel");
            services::runner::run_tests(pattern.map(|s| s.as_str()), parallel, state).await?;
        }

        "build" => {
            let release = matches.get_flag("release");
            let features = matches.get_many::<String>("features")
                .map(|vals| vals.cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            services::builder::build_project(release, features, state).await?;
        }

        "format" => {
            let path = matches.get_one::<String>("path").unwrap();
            let check = matches.get_flag("check");
            services::formatter::format_code(path, check, state).await?;
        }

        "metrics" => {
            let format = matches.get_one::<String>("format").unwrap();
            display_metrics(format, state).await?;
        }

        "server" => {
            let port = matches.get_one::<u16>("port");
            let host = matches.get_one::<String>("host");
            start_web_server(host, port, state).await?;
        }

        "repl" => {
            // Note: REPL handled directly from CLI main to avoid recursion
            println!("REPL should be started from main CLI, not as a subcommand");
        }

        // Legacy commands - delegate to services.
        "greet" => {
            let name = matches.get_one::<String>("name").unwrap();
            services::legacy::greet_command(name).await?;
        }

        "count" => {
            let number = *matches.get_one::<i32>("number").unwrap();
            services::legacy::count_command(number).await?;
        }

        "math" => {
            let a = *matches.get_one::<i32>("a").unwrap();
            let b = *matches.get_one::<i32>("b").unwrap();
            services::legacy::math_command(a, b).await?;
        }

        "file" => {
            let content = matches.get_one::<String>("content").unwrap();
            services::legacy::file_command(content).await?;
        }

        "serialize" => {
            let format = matches.get_one::<String>("format").unwrap();
            let data = matches.get_one::<String>("data").unwrap();
            services::legacy::serialize_command(format, data).await?;
        }

        "crypto" => {
            let algorithm = matches.get_one::<String>("algorithm").unwrap();
            let data = matches.get_one::<String>("data").unwrap();
            services::legacy::crypto_command(algorithm, data).await?;
        }

        "time" => {
            let library = matches.get_one::<String>("library").unwrap();
            let operation = matches.get_one::<String>("operation").unwrap();
            services::legacy::time_command(library, operation).await?;
        }

        "regex" => {
            let pattern = matches.get_one::<String>("pattern").unwrap();
            let text = matches.get_one::<String>("text").unwrap();
            services::legacy::regex_command(pattern, text).await?;
        }

        "async" => {
            let operation = matches.get_one::<String>("operation").unwrap();
            let count = *matches.get_one::<usize>("count").unwrap();
            services::legacy::async_command(operation, count).await?;
        }

        "parallel" => {
            let items = *matches.get_one::<usize>("items").unwrap();
            let threads = matches.get_one::<usize>("threads").cloned();
            services::legacy::parallel_command(items, threads).await?;
        }

        _ => {
            eprintln!("Unknown command: {}", name);
            std::process::exit(1);
        }
    }

    Ok(())
}

// Display metrics in requested format.
async fn display_metrics(format: &str, state: &AppState) -> crate::Result<()> {
    let metrics = state.metrics_snapshot().await;

    match format {
        "json" => {
            // let json = rmx::serde_json::to_string_pretty(&metrics)?;
            // println!("{}", json);
            println!("{{\"status\": \"metrics unavailable in json format\"}}")
        }
        "toml" => {
            // let toml = rmx::toml::to_string_pretty(&metrics)?;
            // println!("{}", toml);
            println!("status = \"metrics unavailable in toml format\"");
        }
        _ => {
            println!("=== Rustmax Suite Metrics ===");
            println!("Uptime: {} seconds", metrics.uptime_secs);
            println!("Requests: {} total, {} success, {} failed",
                metrics.requests_total, metrics.requests_success, metrics.requests_failed);
            println!("Operations: {}", metrics.operations_total);
            println!("Cache: {} hits, {} misses, {} entries ({} bytes)",
                metrics.cache_hits, metrics.cache_misses, metrics.cache_entries, metrics.cache_size);
        }
    }

    Ok(())
}

// Start the web server.
async fn start_web_server(
    host: Option<&String>,
    port: Option<&u16>,
    state: &AppState,
) -> crate::Result<()> {
    let config = state.config().await;
    let host = host.map(|s| s.as_str()).unwrap_or(&config.web.host);
    let port = port.copied().unwrap_or(config.web.port);

    println!("Starting web server at http://{}:{}", host, port);
    println!("Press Ctrl+C to stop...");

    crate::web::serve(host, port, state.clone()).await
}