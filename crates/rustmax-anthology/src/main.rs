//! CLI tool for managing the Rust anthology.

use rmx::prelude::*;
use rmx::clap::{Parser, Subcommand};
use rmx::std::path::PathBuf;
use rustmax_anthology::*;

#[derive(Parser)]
#[command(name = "anthology")]
#[command(about = "Manage the Rust anthology collection")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to metadata directory.
    #[arg(long, default_value = "metadata")]
    metadata_dir: PathBuf,

    /// Path to fetched content directory.
    #[arg(long, default_value = "fetched")]
    fetched_dir: PathBuf,

    /// Path to book directory.
    #[arg(long, default_value = "book")]
    book_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch a post or all posts.
    Fetch {
        /// Post ID to fetch, or "all" for all posts.
        id: String,
    },

    /// Extract content from fetched HTML.
    Extract {
        /// Post ID to extract, or "all" for all posts.
        id: String,
    },

    /// Convert extracted HTML to markdown.
    ToMarkdown {
        /// Post ID to convert, or "all" for all posts.
        id: String,
    },

    /// Process a post through the full pipeline.
    Process {
        /// Post ID to process, or "all" for all posts.
        id: String,
    },

    /// List all posts.
    List,

    /// Show status of posts.
    Status,

    /// Build the mdbook from processed posts.
    Build,

    /// Generate HTML index for the fetched directory.
    GenerateIndex,
}

fn main() -> AnyResult<()> {
    rmx::extras::init();

    let cli = Cli::parse();

    // Load posts metadata.
    let posts_path = cli.metadata_dir.join("posts.toml");
    let collection = metadata::PostCollection::load(&posts_path)
        .context("Failed to load posts.toml")?;

    match cli.command {
        Commands::Fetch { id } => {
            if id == "all" {
                for post in &collection.posts {
                    if let Err(e) = fetch::fetch_post(post, &cli.fetched_dir) {
                        error!("Failed to fetch {}: {}", post.id, e);
                    }
                }
            } else {
                let post = collection.find(&id)
                    .context(format!("Post not found: {}", id))?;
                fetch::fetch_post(post, &cli.fetched_dir)?;
            }
        }

        Commands::Extract { id } => {
            if id == "all" {
                for post in &collection.posts {
                    if let Err(e) = extract::extract_post(post, &cli.fetched_dir) {
                        error!("Failed to extract {}: {}", post.id, e);
                    }
                }
            } else {
                let post = collection.find(&id)
                    .context(format!("Post not found: {}", id))?;
                extract::extract_post(post, &cli.fetched_dir)?;
            }
        }

        Commands::ToMarkdown { id } => {
            if id == "all" {
                for post in &collection.posts {
                    if let Err(e) = markdown::to_markdown(post, &cli.fetched_dir) {
                        error!("Failed to convert {}: {}", post.id, e);
                    }
                }
            } else {
                let post = collection.find(&id)
                    .context(format!("Post not found: {}", id))?;
                markdown::to_markdown(post, &cli.fetched_dir)?;
            }
        }

        Commands::Process { id } => {
            if id == "all" {
                let mut success_count = 0;
                let mut failed_ids = Vec::new();

                for post in &collection.posts {
                    if process_post(post, &cli.fetched_dir) {
                        success_count += 1;
                    } else {
                        failed_ids.push(post.id.clone());
                    }
                }

                println!();
                println!("=== Processing Summary ===");
                println!("Successfully processed: {}/{}", success_count, collection.posts.len());
                if !failed_ids.is_empty() {
                    println!("Failed: {}", failed_ids.join(", "));
                }
                println!();
                println!("Output location: {}", cli.fetched_dir.display());
                println!("  Each post directory contains:");
                println!("    - raw.html        (fetched HTML)");
                println!("    - fetch-info.toml (fetch metadata)");
                println!("    - extracted.html  (extracted content)");
                println!("    - content.md      (final markdown)");
            } else {
                let post = collection.find(&id)
                    .context(format!("Post not found: {}", id))?;
                if process_post(post, &cli.fetched_dir) {
                    let post_dir = cli.fetched_dir.join(&post.id);
                    println!();
                    println!("Output files:");
                    println!("  {}/raw.html", post_dir.display());
                    println!("  {}/fetch-info.toml", post_dir.display());
                    println!("  {}/extracted.html", post_dir.display());
                    println!("  {}/content.md", post_dir.display());
                }
            }
        }

        Commands::List => {
            let mut posts = collection.posts.clone();
            posts.sort_by(|a, b| a.id.cmp(&b.id));
            for post in &posts {
                println!("{}: {} by {}", post.id, post.title, post.author);
            }
        }

        Commands::Status => {
            println!("Post Status:");
            println!("{:<30} {:<10} {:<10} {:<10}", "ID", "Fetched", "Extracted", "Markdown");
            println!("{}", "-".repeat(70));

            let mut posts = collection.posts.clone();
            posts.sort_by(|a, b| a.id.cmp(&b.id));
            let mut complete_count = 0;
            for post in &posts {
                let fetched = if fetch::is_fetched(post, &cli.fetched_dir) { "✓" } else { "✗" };
                let extracted = if extract::is_extracted(post, &cli.fetched_dir) { "✓" } else { "✗" };
                let markdown = if markdown::has_markdown(post, &cli.fetched_dir) { "✓" } else { "✗" };

                if fetched == "✓" && extracted == "✓" && markdown == "✓" {
                    complete_count += 1;
                }

                println!("{:<30} {:<10} {:<10} {:<10}", post.id, fetched, extracted, markdown);
            }

            println!();
            println!("Complete: {}/{}", complete_count, posts.len());
            println!();
            println!("Output location: {}", cli.fetched_dir.display());
            println!("  Each post directory contains:");
            println!("    - raw.html        (fetched HTML)");
            println!("    - fetch-info.toml (fetch metadata)");
            println!("    - extracted.html  (extracted content)");
            println!("    - content.md      (final markdown)");
        }

        Commands::Build => {
            build::build_book(&collection, &cli.fetched_dir, &cli.book_dir)?;
        }

        Commands::GenerateIndex => {
            index::generate_index(&collection, &cli.fetched_dir)?;
        }
    }

    Ok(())
}

/// Process a single post through the full pipeline.
///
/// Returns true if successful, false otherwise.
fn process_post(post: &Post, fetched_dir: &PathBuf) -> bool {
    info!("Processing post: {}", post.id);

    if let Err(e) = fetch::fetch_post(post, fetched_dir) {
        error!("Failed to fetch {}: {}", post.id, e);
        return false;
    }

    if let Err(e) = extract::extract_post(post, fetched_dir) {
        error!("Failed to extract {}: {}", post.id, e);
        return false;
    }

    if let Err(e) = markdown::to_markdown(post, fetched_dir) {
        error!("Failed to convert {}: {}", post.id, e);
        return false;
    }

    info!("Successfully processed: {}", post.id);
    true
}
