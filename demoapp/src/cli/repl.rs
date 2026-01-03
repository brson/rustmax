//! Interactive REPL for collection queries.

use rustmax::prelude::*;
use rustmax::rustyline::error::ReadlineError;
use rustmax::rustyline::{DefaultEditor, Config, EditMode};
use rustmax::termcolor::{ColorChoice, StandardStream, WriteColor, ColorSpec, Color};
use std::io::Write;

use crate::collection::{Collection, Config as CollectionConfig};
use crate::search::SearchIndex;
use crate::Result;

/// Run the interactive REPL.
pub fn run_repl(collection: &Collection, config: &CollectionConfig) -> Result<()> {
    let rl_config = Config::builder()
        .edit_mode(EditMode::Emacs)
        .auto_add_history(true)
        .build();

    let mut rl: DefaultEditor = DefaultEditor::with_config(rl_config)
        .map_err(|e| crate::Error::Other(e.into()))?;

    // Build search index for queries.
    let index = SearchIndex::build(collection);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    println!("Anthology REPL - type 'help' for commands, 'quit' to exit");
    println!("Collection: {} ({} documents)", config.collection.title, collection.documents.len());
    println!();

    loop {
        match rl.readline("anthology> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                let cmd = parts[0];
                let arg = parts.get(1).copied().unwrap_or("");

                match cmd {
                    "help" | "?" => print_help(),
                    "quit" | "exit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "list" | "ls" => cmd_list(collection, &mut stdout)?,
                    "drafts" => cmd_drafts(collection, &mut stdout)?,
                    "show" => cmd_show(collection, arg, &mut stdout)?,
                    "search" | "find" => cmd_search(&index, arg, &mut stdout)?,
                    "tags" => cmd_tags(collection, &mut stdout)?,
                    "by-tag" | "tag" => cmd_by_tag(collection, arg, &mut stdout)?,
                    "stats" => cmd_stats(collection, &mut stdout)?,
                    "recent" => cmd_recent(collection, &mut stdout)?,
                    "files" => cmd_files(collection, arg, &mut stdout)?,
                    _ => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        write!(stdout, "Unknown command")?;
                        stdout.reset()?;
                        writeln!(stdout, ": {}. Type 'help' for available commands.", cmd)?;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                return Err(crate::Error::Other(err.into()));
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  list, ls           List all published documents");
    println!("  drafts             List draft documents");
    println!("  show <slug>        Show document details");
    println!("  search <query>     Search documents");
    println!("  tags               List all tags");
    println!("  by-tag <tag>       List documents with tag");
    println!("  files [pattern]    List files matching glob pattern");
    println!("  stats              Show collection statistics");
    println!("  recent             Show 5 most recent documents");
    println!("  help, ?            Show this help");
    println!("  quit, exit, q      Exit the REPL");
}

fn cmd_list(collection: &Collection, stdout: &mut StandardStream) -> Result<()> {
    let docs = collection.published();
    if docs.is_empty() {
        println!("No published documents.");
        return Ok(());
    }

    for doc in &docs {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{:20}", doc.slug())?;
        stdout.reset()?;
        write!(stdout, " ")?;

        if let Some(date) = doc.frontmatter.date {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            write!(stdout, "{}", date)?;
            stdout.reset()?;
            write!(stdout, " ")?;
        }

        writeln!(stdout, "{}", doc.frontmatter.title)?;
    }

    println!("\n{} documents", docs.len());
    Ok(())
}

fn cmd_drafts(collection: &Collection, stdout: &mut StandardStream) -> Result<()> {
    let drafts: Vec<_> = collection.documents.iter()
        .filter(|d| d.frontmatter.draft)
        .collect();

    if drafts.is_empty() {
        println!("No draft documents.");
        return Ok(());
    }

    for doc in &drafts {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
        write!(stdout, "[DRAFT] ")?;
        stdout.reset()?;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{:20}", doc.slug())?;
        stdout.reset()?;

        writeln!(stdout, " {}", doc.frontmatter.title)?;
    }

    println!("\n{} drafts", drafts.len());
    Ok(())
}

fn cmd_show(collection: &Collection, slug: &str, stdout: &mut StandardStream) -> Result<()> {
    if slug.is_empty() {
        println!("Usage: show <slug>");
        return Ok(());
    }

    let doc = collection.documents.iter().find(|d| d.slug() == slug);

    match doc {
        Some(doc) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
            writeln!(stdout, "{}", doc.frontmatter.title)?;
            stdout.reset()?;

            println!("Slug:     {}", doc.slug());

            if let Some(date) = doc.frontmatter.date {
                println!("Date:     {}", date);
            }

            if !doc.frontmatter.tags.is_empty() {
                println!("Tags:     {}", doc.frontmatter.tags.join(", "));
            }

            println!("Draft:    {}", doc.frontmatter.draft);
            println!("Words:    {}", doc.word_count());
            println!("Reading:  {} min", doc.reading_time());
            println!("Hash:     {}...", &doc.content_hash[..16]);
            println!("Path:     {}", doc.source_path.display());

            if let Some(desc) = &doc.frontmatter.description {
                println!("\n{}", desc);
            }
        }
        None => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            write!(stdout, "Not found")?;
            stdout.reset()?;
            writeln!(stdout, ": {}", slug)?;
        }
    }

    Ok(())
}

fn cmd_search(index: &SearchIndex, query: &str, stdout: &mut StandardStream) -> Result<()> {
    if query.is_empty() {
        println!("Usage: search <query>");
        return Ok(());
    }

    let results = index.search(query);

    if results.is_empty() {
        println!("No results for '{}'", query);
        return Ok(());
    }

    for result in &results {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{:20}", result.slug)?;
        stdout.reset()?;

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, " [{}]", result.score)?;
        stdout.reset()?;

        writeln!(stdout, " {}", result.title)?;
    }

    println!("\n{} results", results.len());
    Ok(())
}

fn cmd_tags(collection: &Collection, stdout: &mut StandardStream) -> Result<()> {
    let tags = collection.tags();

    if tags.is_empty() {
        println!("No tags found.");
        return Ok(());
    }

    for tag in &tags {
        let count = collection.by_tag(tag).len();

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(stdout, "{}", tag)?;
        stdout.reset()?;

        writeln!(stdout, " ({})", count)?;
    }

    println!("\n{} tags", tags.len());
    Ok(())
}

fn cmd_by_tag(collection: &Collection, tag: &str, stdout: &mut StandardStream) -> Result<()> {
    if tag.is_empty() {
        println!("Usage: by-tag <tag>");
        return Ok(());
    }

    let docs = collection.by_tag(tag);

    if docs.is_empty() {
        println!("No documents with tag '{}'", tag);
        return Ok(());
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(stdout, "Tag: {}", tag)?;
    stdout.reset()?;
    println!();

    for doc in &docs {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{:20}", doc.slug())?;
        stdout.reset()?;
        writeln!(stdout, " {}", doc.frontmatter.title)?;
    }

    println!("\n{} documents", docs.len());
    Ok(())
}

fn cmd_stats(collection: &Collection, stdout: &mut StandardStream) -> Result<()> {
    let published = collection.published();
    let draft_count = collection.documents.iter()
        .filter(|d| d.frontmatter.draft)
        .count();

    // Use itertools fold_ok pattern for clean aggregation.
    let (total_words, total_reading): (usize, usize) = collection.documents.iter()
        .map(|d| (d.word_count(), d.reading_time()))
        .fold((0, 0), |(w, r), (dw, dr)| (w + dw, r + dr));

    let tags = collection.tags();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(stdout, "Collection Statistics")?;
    stdout.reset()?;
    println!();

    println!("Total documents:  {}", collection.documents.len());
    println!("Published:        {}", published.len());
    println!("Drafts:           {}", draft_count);
    println!("Total words:      {}", total_words);
    println!("Reading time:     {} min", total_reading);
    println!("Unique tags:      {}", tags.len());

    if let Some(newest) = published.first() {
        println!();
        println!("Most recent:      {} ({})",
            newest.frontmatter.title,
            newest.frontmatter.date.map(|d| d.to_string()).unwrap_or_default()
        );
    }

    Ok(())
}

fn cmd_recent(collection: &Collection, stdout: &mut StandardStream) -> Result<()> {
    let docs = collection.published();

    if docs.is_empty() {
        println!("No published documents.");
        return Ok(());
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(stdout, "Recent Documents")?;
    stdout.reset()?;
    println!();

    for doc in docs.iter().take(5) {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{:20}", doc.slug())?;
        stdout.reset()?;
        write!(stdout, " ")?;

        if let Some(date) = doc.frontmatter.date {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            write!(stdout, "{}", date)?;
            stdout.reset()?;
            write!(stdout, " ")?;
        }

        writeln!(stdout, "{}", doc.frontmatter.title)?;
    }

    Ok(())
}

fn cmd_files(collection: &Collection, pattern: &str, stdout: &mut StandardStream) -> Result<()> {
    use rustmax::glob::glob;

    // Default pattern if none provided.
    let pattern = if pattern.is_empty() {
        "content/**/*.md"
    } else {
        pattern
    };

    let full_pattern = collection.root.join(pattern);
    let pattern_str = full_pattern.to_string_lossy();

    let entries = glob(&pattern_str).map_err(|e| {
        crate::Error::config(format!("Invalid glob pattern: {}", e))
    })?;

    let mut count = 0;
    for entry in entries {
        match entry {
            Ok(path) => {
                let display_path = path.strip_prefix(&collection.root)
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
