//! CLI command definitions.

use rustmax::prelude::*;
use clap::{Parser, Subcommand, ValueEnum};
use rustmax::log::info;
use std::path::PathBuf;

use crate::{Result, Error};
use crate::collection::Config;

/// Anthology: A document publishing platform.
#[derive(Parser, Debug)]
#[command(name = "anthology")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a new collection.
    Init {
        /// Directory to initialize (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Build the collection to static output.
    Build {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output directory (defaults to 'output').
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Include draft documents.
        #[arg(long)]
        drafts: bool,
    },

    /// Start a development server with live reload.
    Serve {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Port to listen on.
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Include draft documents.
        #[arg(long)]
        drafts: bool,
    },

    /// Validate documents in the collection.
    Check {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Create a new document.
    New {
        /// Title of the document.
        title: String,

        /// Collection directory (defaults to current directory).
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },

    /// Rebuild the search index.
    Index {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Export the collection to a different format.
    Export {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format.
        #[arg(short, long, default_value = "json")]
        format: ExportFormat,

        /// Output file.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ExportFormat {
    Json,
    Rss,
    Sitemap,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        // Initialize logging.
        let log_level = if self.verbose { "debug" } else { "info" };
        rustmax::env_logger::Builder::from_env(
            rustmax::env_logger::Env::default().default_filter_or(log_level)
        ).init();

        match self.command {
            Command::Init { path } => cmd_init(path),
            Command::Build { path, output, drafts } => cmd_build(path, output, drafts),
            Command::Serve { path, port, drafts } => cmd_serve(path, port, drafts),
            Command::Check { path } => cmd_check(path),
            Command::New { title, path } => cmd_new(title, path),
            Command::Index { path } => cmd_index(path),
            Command::Export { path, format, output } => cmd_export(path, format, output),
        }
    }
}

fn cmd_init(path: PathBuf) -> Result<()> {
    use std::fs;
    use rustmax::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    let config_path = path.join("anthology.toml");

    if config_path.exists() {
        return Err(Error::config(format!(
            "Collection already exists at {}",
            path.display()
        )));
    }

    // Create directory structure.
    fs::create_dir_all(&path)?;
    fs::create_dir_all(path.join("content"))?;
    fs::create_dir_all(path.join("templates"))?;
    fs::create_dir_all(path.join("static"))?;

    // Write default config.
    let default_config = Config::default();
    let config_toml = rustmax::toml::to_string_pretty(&default_config)
        .map_err(|e| Error::config(e.to_string()))?;
    fs::write(&config_path, config_toml)?;

    // Write default template.
    let default_template = include_str!("../../templates/default.html");
    fs::write(path.join("templates/default.html"), default_template)?;

    // Write example document.
    let example_doc = r#"---
title = "Welcome to Anthology"
date = "2024-01-01"
tags = ["welcome", "example"]
---

# Welcome

This is your first document. Edit or delete it and start writing!
"#;
    fs::write(path.join("content/welcome.md"), example_doc)?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(stdout, "Created")?;
    stdout.reset()?;
    writeln!(stdout, " new collection at {}", path.display())?;

    Ok(())
}

fn cmd_build(path: PathBuf, output: Option<PathBuf>, drafts: bool) -> Result<()> {
    info!("Building collection at {}", path.display());

    let config = Config::load(&path)?;
    let output_dir = output.unwrap_or_else(|| path.join(&config.build.output_dir));

    let collection = crate::collection::Collection::load(&path, &config)?;
    info!("Found {} documents", collection.documents.len());

    crate::build::build(&collection, &config, &output_dir, drafts)?;

    info!("Build complete: {}", output_dir.display());
    Ok(())
}

fn cmd_serve(path: PathBuf, port: u16, drafts: bool) -> Result<()> {
    info!("Starting server for collection at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    crate::serve::serve(collection, config, port, drafts)
}

fn cmd_check(path: PathBuf) -> Result<()> {
    use rustmax::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    info!("Checking collection at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut errors = 0;

    for doc in &collection.documents {
        if let Err(e) = doc.validate() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(stdout, "Error")?;
            stdout.reset()?;
            writeln!(stdout, " {}: {}", doc.source_path.display(), e)?;
            errors += 1;
        }
    }

    if errors == 0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(stdout, "OK")?;
        stdout.reset()?;
        writeln!(stdout, " {} documents validated", collection.documents.len())?;
    } else {
        return Err(Error::build(format!("{} validation errors", errors)));
    }

    Ok(())
}

fn cmd_new(title: String, path: PathBuf) -> Result<()> {
    use rustmax::jiff::Zoned;
    use rustmax::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    let _config = Config::load(&path)?;
    let content_dir = path.join("content");

    // Generate filename from title.
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    let filename = format!("{}.md", slug);
    let filepath = content_dir.join(&filename);

    if filepath.exists() {
        return Err(Error::document(&filepath, "file already exists"));
    }

    let now = Zoned::now();
    let date = now.strftime("%Y-%m-%d").to_string();

    let content = format!(
        r#"---
title = "{title}"
date = "{date}"
draft = true
tags = []
---

Write your content here.
"#
    );

    std::fs::write(&filepath, content)?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(stdout, "Created")?;
    stdout.reset()?;
    writeln!(stdout, " {}", filepath.display())?;

    Ok(())
}

fn cmd_index(path: PathBuf) -> Result<()> {
    info!("Rebuilding search index at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    crate::search::build_index(&collection, &path)?;

    info!("Index rebuilt");
    Ok(())
}

fn cmd_export(path: PathBuf, format: ExportFormat, output: Option<PathBuf>) -> Result<()> {
    use std::io::Write;

    info!("Exporting collection at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    let content = match format {
        ExportFormat::Json => {
            rustmax::serde_json::to_string_pretty(&collection.to_export())?
        }
        ExportFormat::Rss => {
            crate::build::generate_rss(&collection, &config)?
        }
        ExportFormat::Sitemap => {
            crate::build::generate_sitemap(&collection, &config)?
        }
    };

    match output {
        Some(path) => {
            std::fs::write(&path, content)?;
            info!("Exported to {}", path.display());
        }
        None => {
            std::io::stdout().write_all(content.as_bytes())?;
        }
    }

    Ok(())
}
