//! CLI command definitions.

use rmx::prelude::*;
use rmx::log::info;
use std::path::PathBuf;

use crate::{Result, Error};
use crate::collection::Config;

/// Anthology: A document publishing platform.
#[rmx::derive(Parser, Debug)]
#[command(name = "anthology")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[rmx::derive(Subcommand, Debug)]
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

        /// Compress output files (create .gz versions).
        #[arg(long)]
        compress: bool,

        /// Use incremental build (skip unchanged documents).
        #[arg(short, long)]
        incremental: bool,

        /// Show progress bars during build.
        #[arg(short, long)]
        progress: bool,
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

        /// Open the site in a browser once the server is listening.
        ///
        /// Overrides `[server] open` in the collection's config.
        #[arg(long)]
        open: bool,
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

    /// Start an interactive REPL for querying the collection.
    Repl {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// List files matching a glob pattern.
    Files {
        /// Collection directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Glob pattern to match (e.g., "content/**/*.md").
        #[arg(short, long, default_value = "content/**/*.md")]
        pattern: String,
    },

    /// Fetch content from a remote URL.
    Fetch {
        /// URL to fetch content from.
        url: String,

        /// Destination path (relative to collection root).
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Collection directory (defaults to current directory).
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

#[rmx::derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ExportFormat {
    Json,
    Rss,
    Atom,
    JsonFeed,
    Sitemap,
    Epub,
    /// Gzipped tarball of the built output directory.
    Tar,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        // Initialize logging.
        let log_level = if self.verbose { "debug" } else { "info" };
        rmx::env_logger::Builder::from_env(
            rmx::env_logger::Env::default().default_filter_or(log_level)
        ).init();

        match self.command {
            Command::Init { path } => cmd_init(path),
            Command::Build { path, output, drafts, compress, incremental, progress } => cmd_build(path, output, drafts, compress, incremental, progress),
            Command::Serve { path, port, drafts, open } => cmd_serve(path, port, drafts, open),
            Command::Check { path } => cmd_check(path),
            Command::New { title, path } => cmd_new(title, path),
            Command::Index { path } => cmd_index(path),
            Command::Export { path, format, output } => cmd_export(path, format, output),
            Command::Repl { path } => cmd_repl(path),
            Command::Files { path, pattern } => cmd_files(path, pattern),
            Command::Fetch { url, output, path } => cmd_fetch(url, output, path),
        }
    }
}

fn cmd_init(path: PathBuf) -> Result<()> {
    use std::fs;
    use rmx::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
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
    let config_toml = rmx::toml::to_string_pretty(&default_config)
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

fn cmd_build(path: PathBuf, output: Option<PathBuf>, drafts: bool, compress: bool, incremental: bool, progress: bool) -> Result<()> {
    if !progress {
        info!("Building collection at {}", path.display());
    }

    let config = Config::load(&path)?;
    let output_dir = output.unwrap_or_else(|| path.join(&config.build.output_dir));

    let collection = crate::collection::Collection::load(&path, &config)?;
    if !progress {
        info!("Found {} documents", collection.documents.len());
    }

    if incremental {
        if progress {
            crate::build::build_incremental_with_progress(&collection, &config, &output_dir, drafts)?;
        } else {
            crate::build::build_incremental(&collection, &config, &output_dir, drafts)?;
        }
    } else {
        if progress {
            crate::build::build_with_progress(&collection, &config, &output_dir, drafts)?;
        } else {
            crate::build::build(&collection, &config, &output_dir, drafts)?;
        }
    }

    if compress {
        if progress {
            let compress_pb = crate::build::compress_spinner();
            crate::build::compress_output(&output_dir)?;
            crate::build::finish_with_check(&compress_pb, "Output compressed");
        } else {
            info!("Compressing output files...");
            crate::build::compress_output(&output_dir)?;
        }
    }

    if !progress {
        info!("Build complete: {}", output_dir.display());
    } else {
        println!("Build complete: {}", output_dir.display());
    }
    Ok(())
}

fn cmd_serve(path: PathBuf, port: u16, drafts: bool, open: bool) -> Result<()> {
    info!("Starting server for collection at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    let open = open || config.server.open;
    crate::serve::serve_with_options(collection, config, port, drafts, open)
}

fn cmd_check(path: PathBuf) -> Result<()> {
    use rmx::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    info!("Checking collection at {}", path.display());

    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut errors = 0;
    let mut warnings = 0;

    for doc in &collection.documents {
        if let Err(e) = doc.validate() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(stdout, "error")?;
            stdout.reset()?;
            writeln!(stdout, " {}: {}", doc.source_path.display(), e)?;
            errors += 1;
        }

        for lint in crate::lint::check_document(doc) {
            let color = match lint.level {
                crate::lint::Level::Error => Color::Red,
                crate::lint::Level::Warning => Color::Yellow,
            };
            stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
            write!(stdout, "{}", lint.level)?;
            stdout.reset()?;
            writeln!(
                stdout,
                " {}:{}: {}",
                lint.path.display(),
                lint.line,
                lint.message,
            )?;

            match lint.level {
                crate::lint::Level::Error => errors += 1,
                crate::lint::Level::Warning => warnings += 1,
            }
        }
    }

    if errors > 0 {
        return Err(Error::check(errors));
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(stdout, "ok")?;
    stdout.reset()?;
    write!(stdout, " {} documents validated", collection.documents.len())?;
    if warnings > 0 {
        write!(stdout, ", {} warnings", warnings)?;
    }
    writeln!(stdout)?;

    Ok(())
}

fn cmd_new(title: String, path: PathBuf) -> Result<()> {
    use rmx::jiff::Zoned;
    use rmx::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
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

    // EPUB and tar are binary, and go straight to a file rather than stdout.
    if format == ExportFormat::Epub {
        let epub_path = output.unwrap_or_else(|| path.join("output.epub"));
        let epub_config = crate::export::EpubConfig::default();
        crate::export::generate_epub(&collection, &config, &epub_path, &epub_config)?;
        info!("Exported EPUB to {}", epub_path.display());
        return Ok(());
    }

    if format == ExportFormat::Tar {
        let options = crate::export::ArchiveOptions::default();
        let archive_path = output
            .unwrap_or_else(|| path.join(crate::export::default_archive_name(&options)));
        let built = path.join(&config.build.output_dir);

        let stats = crate::export::archive_directory(&built, &archive_path, &options)?;
        info!(
            "Archived {} files to {} ({} bytes)",
            stats.files,
            archive_path.display(),
            stats.archive_bytes,
        );
        return Ok(());
    }

    let content = match format {
        ExportFormat::Json => {
            rmx::serde_json::to_string_pretty(&collection.to_export())?
        }
        ExportFormat::Rss => {
            crate::build::generate_rss(&collection, &config)?
        }
        ExportFormat::Atom => {
            crate::feeds::generate_atom(&collection, &config)?
        }
        ExportFormat::JsonFeed => {
            crate::feeds::generate_json_feed(&collection, &config)?
        }
        ExportFormat::Sitemap => {
            crate::build::generate_sitemap(&collection, &config)?
        }
        // Both are written to a file above and never reach here.
        ExportFormat::Epub | ExportFormat::Tar => bug!("binary export format"),
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

fn cmd_repl(path: PathBuf) -> Result<()> {
    let config = Config::load(&path)?;
    let collection = crate::collection::Collection::load(&path, &config)?;

    super::repl::run_repl(&collection, &config)
}

fn cmd_files(path: PathBuf, pattern: String) -> Result<()> {
    use rmx::glob::glob;
    use rmx::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    let full_pattern = path.join(&pattern);
    let pattern_str = full_pattern.to_string_lossy();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut count = 0;

    let entries = glob(&pattern_str).map_err(|e| Error::config(format!("Invalid glob pattern: {}", e)))?;

    for entry in entries {
        match entry {
            Ok(path) => {
                // Get relative path from the collection root.
                let display_path = path.strip_prefix(&std::env::current_dir()?)
                    .unwrap_or(&path);

                if path.is_file() {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                    writeln!(stdout, "{}", display_path.display())?;
                    stdout.reset()?;
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                    writeln!(stdout, "{}/", display_path.display())?;
                    stdout.reset()?;
                }
                count += 1;
            }
            Err(e) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(stdout, "Error: {}", e)?;
                stdout.reset()?;
            }
        }
    }

    println!("\n{} files matched", count);
    Ok(())
}

fn cmd_fetch(url: String, output: Option<PathBuf>, path: PathBuf) -> Result<()> {
    use rmx::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
    use std::io::Write;

    info!("Fetching content from {}", url);

    // Determine output path.
    let dest = if let Some(out) = output {
        path.join(out)
    } else {
        // Extract filename from URL.
        let filename = url
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty() && s.contains('.'))
            .unwrap_or("fetched.md");
        path.join("content").join(filename)
    };

    // Run async fetch in tokio runtime.
    let rt = rmx::tokio::runtime::Runtime::new()
        .map_err(|e| Error::Other(e.into()))?;

    rt.block_on(async {
        crate::remote::fetch_document(&url, &dest).await
    })?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(stdout, "Fetched")?;
    stdout.reset()?;
    writeln!(stdout, " {} -> {}", url, dest.display())?;

    Ok(())
}
