Command line argument parsing.

- Crate [`::clap`].
- [docs.rs](https://docs.rs/clap)
- [crates.io](https://crates.io/crates/clap)
- [GitHub](https://github.com/clap-rs/clap)

---

`clap` is a powerful and feature-rich command-line argument parser
that provides a polished CLI experience out-of-the-box.
It supports two primary API approaches: the derive API for simplicity
and the builder API for maximum flexibility.

## Examples

Simple command-line tool using the derive API:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A simple greeting tool")]
struct Args {
    /// Name to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "1")]
    count: u8,
}

fn main() {
    // Parse from custom args for testing
    let args = Args::parse_from(&["prog", "--name", "World"]);

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
```

Application with subcommands:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "File management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new file
    Create {
        /// File name to create
        name: String,
    },
    /// List files
    List {
        /// Show hidden files
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    // Parse from custom args for testing
    let cli = Cli::parse_from(&["prog", "create", "test.txt"]);

    match cli.command {
        Commands::Create { name } => {
            println!("Creating file: {}", name);
        }
        Commands::List { all } => {
            if all {
                println!("Listing all files including hidden");
            } else {
                println!("Listing visible files");
            }
        }
    }
}
```