use rmx::json5;
use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::xshell::{Shell, cmd};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Books {
    books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Book {
    slug: String,
    name: String,
    repo: String,
    #[serde(default)]
    book_path: Option<String>,
    #[serde(default)]
    needs_nightly: bool,
    commit: String,
}

#[derive(Debug, Clone)]
struct BookBuildResult {
    book: Book,
    success: bool,
    error_message: Option<String>,
}

pub fn list_library(root: &Path) -> AnyResult<()> {
    for book in load(root)?.books {
        println!("{}", book.slug);
    }
    Ok(())
}

pub fn build_library(root: &Path, no_fetch: bool, generate_library: bool) -> AnyResult<()> {
    let books = load(root)?.books;
    let results = build_books(&books, no_fetch);
    copy_books_to_library(&books)?;
    if generate_library {
        crate::library_gen::generate_library_page()?;
    }
    check_results(&results)
}

pub fn build_one_book(root: &Path, slug: &str, no_fetch: bool) -> AnyResult<()> {
    let book: Vec<Book> = load(root)?
        .books
        .into_iter()
        .filter(|b| b.slug == slug)
        .collect();
    if book.is_empty() {
        return Err(anyhow!("unknown book '{slug}'"));
    }
    let results = build_books(&book, no_fetch);
    copy_books_to_library(&book)?;
    check_results(&results)
}

pub fn refresh_library(root: &Path) -> AnyResult<()> {
    let books = load(root)?.books;
    println!("Refreshing all library repositories...");
    for book in &books {
        if let Err(e) = get_repo(book) {
            eprintln!("Failed to refresh {}: {}", book.slug, e);
        }
    }
    println!("Library refresh complete");
    Ok(())
}

pub fn refresh_one_book(root: &Path, slug: &str) -> AnyResult<()> {
    let book: Vec<Book> = load(root)?
        .books
        .into_iter()
        .filter(|b| b.slug == slug)
        .collect();
    if book.is_empty() {
        return Err(anyhow!("unknown book '{slug}'"));
    }
    println!("Refreshing {} repository...", slug);
    get_repo(&book[0])?;
    println!("{} refresh complete", slug);
    Ok(())
}

fn build_books(books: &[Book], no_fetch: bool) -> Vec<BookBuildResult> {
    let mut results: Vec<BookBuildResult> = Vec::new();

    // Step 1: Clone/update repos (unless skipped).
    if !no_fetch {
        for book in books {
            println!("Processing: {} - {}", book.slug, book.name);
            if let Err(e) = get_repo(book) {
                eprintln!("Failed to clone/update {}: {}", book.slug, e);
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: false,
                    error_message: Some(format!("Clone/update failed: {}", e)),
                });
            }
        }
    } else {
        println!("Skipping git clone/fetch operations");
    }

    // Step 2: Prepare and build.
    for book in books {
        if results.iter().any(|r| r.book.slug == book.slug && !r.success) {
            continue;
        }

        println!("Processing: {} - {}", book.slug, book.name);

        if let Err(e) = prepare_book(book) {
            eprintln!("Failed to prepare {}: {}", book.slug, e);
            results.push(BookBuildResult {
                book: book.clone(),
                success: false,
                error_message: Some(format!("Preparation failed: {}", e)),
            });
            continue;
        }

        match build_book(book) {
            Ok(_) => {
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: true,
                    error_message: None,
                });
            }
            Err(e) => {
                eprintln!("Failed to build {}: {}", book.slug, e);
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: false,
                    error_message: Some(e.to_string()),
                });
            }
        }
    }

    print_build_summary(&results);
    results
}

/// Turn any book build failure into an error so callers don't report success.
fn check_results(results: &[BookBuildResult]) -> AnyResult<()> {
    let failed: Vec<&str> = results
        .iter()
        .filter(|r| !r.success)
        .map(|r| r.book.slug.as_str())
        .collect();

    if !failed.is_empty() {
        bail!("failed to build books: {}", failed.join(", "));
    }

    Ok(())
}

const BOOK_GIT_DIR: &str = "work/books/";

fn book_src_dir(book: &Book) -> String {
    format!("{BOOK_GIT_DIR}/{}", book.slug)
}

fn book_out_dir(book: &Book) -> String {
    if book.slug == "bindgen" {
        format!("{BOOK_GIT_DIR}/{}/book-html", book.slug)
    } else {
        format!("{BOOK_GIT_DIR}/{}/book", book.slug)
    }
}

fn build_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    let book_subdir = book.book_path.as_deref().unwrap_or("");
    let ref build_dir = if book_subdir.is_empty() {
        src_dir.to_string()
    } else {
        format!("{}/{}", src_dir, book_subdir)
    };

    if !fs::exists(build_dir)? {
        return Err(anyhow!("Book directory not found: {}", build_dir));
    }

    let output_dir = book_out_dir(book);

    println!("  Building {}", book.slug);

    if fs::exists(&output_dir)? {
        fs::remove_dir_all(&output_dir)?;
    }

    let input = Path::new(build_dir);
    let output = Path::new(&output_dir);

    crate::rmxbook::build(input, output)?;

    if !fs::exists(format!("{}/index.html", output_dir))? {
        return Err(anyhow!("Build failed - index.html not found"));
    }

    Ok(())
}

fn get_repo(book: &Book) -> AnyResult<()> {
    let ref repo = book.repo;
    let ref dir = book_src_dir(book);
    let ref commit = book.commit;

    fs::create_dir_all(BOOK_GIT_DIR)?;

    let sh = Shell::new()?;
    let need_clone = if fs::exists(dir)? {
        let _pd = sh.push_dir(dir);
        let current_remote = cmd!(sh, "git remote get-url origin").read().unwrap_or_default();
        if current_remote.trim() != repo {
            println!("  Remote mismatch for {}, re-cloning", book.slug);
            println!("    Expected: {}", repo);
            println!("    Found: {}", current_remote.trim());
            drop(_pd);
            fs::remove_dir_all(dir)?;
            true
        } else {
            false
        }
    } else {
        true
    };

    if need_clone {
        println!("  Cloning {} from {} (blobless)", book.slug, repo);
        cmd!(sh, "git clone --filter=blob:none {repo} {dir}").run()?;
    } else {
        println!("  Fetching commit {} for {}", commit, book.slug);
        let _pd = sh.push_dir(dir);
        cmd!(sh, "git fetch origin {commit}").run()?;
    }

    println!("  Checking out commit {} for {}", commit, book.slug);
    let _pd = sh.push_dir(dir);
    cmd!(sh, "git checkout -f {commit}").run()?;

    Ok(())
}

fn print_build_summary(results: &[BookBuildResult]) {
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;

    println!("\nBuild Summary: {} succeeded, {} failed", successful, failed);

    if failed > 0 {
        println!("\nFailed books:");
        for result in results.iter().filter(|r| !r.success) {
            println!("  - {}", result.book.slug);
        }
    }
}

fn prepare_book(book: &Book) -> AnyResult<()> {
    match book.slug.as_str() {
        "rfcs" => prepare_rfcs_book(book),
        "bindgen" => prepare_bindgen_book(book),
        "mdbook" => prepare_mdbook_book(book),
        _ => Ok(())
    }
}

/// Build the RFC book's `src` directory and `SUMMARY.md` from its `text` directory.
///
/// This reimplements upstream's `generate-book.py`. We can't just run that
/// script because it ends by invoking the `mdbook` binary, which isn't
/// installed on the doc build machine and whose output `build_book` throws
/// away anyway. Upstream symlinks `text` into `src`; we copy instead so the
/// build doesn't depend on symlink support.
fn prepare_rfcs_book(book: &Book) -> AnyResult<()> {
    use std::fmt::Write as _;

    println!("  Preparing RFCs book - generating SUMMARY.md");

    let book_dir = PathBuf::from(book_src_dir(book));
    let ref text_dir = book_dir.join("text");
    let ref src_dir = book_dir.join("src");

    // Clear out src to remove stale entries in case the checkout moved.
    if fs::exists(src_dir)? {
        fs::remove_dir_all(src_dir)?;
    }
    fs::create_dir_all(src_dir)?;

    copy_dir_contents(text_dir, src_dir)?;

    for (from, to) in [
        ("compiler_changes.md", "compiler_changes.md"),
        ("lang_changes.md", "lang_changes.md"),
        ("libs_changes.md", "libs_changes.md"),
        ("README.md", "introduction.md"),
    ] {
        fs::copy(book_dir.join(from), src_dir.join(to))?;
    }

    let mut summary = String::new();
    writeln!(summary, "[Introduction](introduction.md)\n")?;
    writeln!(summary, "- [Guidelines for compiler changes](compiler_changes.md)")?;
    writeln!(summary, "- [Guidelines for language changes](lang_changes.md)")?;
    writeln!(summary, "- [Guidelines for library changes](libs_changes.md)")?;
    collect_summary(&mut summary, text_dir, "", 0)?;
    fs::write(src_dir.join("SUMMARY.md"), summary)?;

    println!("  RFCs book preparation complete");
    Ok(())
}

/// Append a sorted `SUMMARY.md` chapter list for one directory of RFCs.
///
/// An RFC that spreads across several pages keeps them in a subdirectory named
/// after the RFC, and those become nested chapters. `link_prefix` is the path
/// of `dir` relative to the book's `src` directory, with a trailing slash.
fn collect_summary(
    summary: &mut String,
    dir: &Path,
    link_prefix: &str,
    depth: usize,
) -> AnyResult<()> {
    use std::fmt::Write as _;

    let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.retain(|entry| entry.file_name().to_string_lossy().ends_with(".md"));
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let name = file_name.strip_suffix(".md").expect("filtered above");
        let indent = "    ".repeat(depth);

        writeln!(summary, "{indent}- [{name}]({link_prefix}{file_name})")?;

        let ref subdir = dir.join(name);
        if subdir.is_dir() {
            collect_summary(summary, subdir, &format!("{link_prefix}{name}/"), depth + 1)?;
        }
    }

    Ok(())
}

fn copy_dir_contents(from: &Path, to: &Path) -> AnyResult<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let ref dest = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            fs::create_dir_all(dest)?;
            copy_dir_contents(&entry.path(), dest)?;
        } else {
            fs::copy(entry.path(), dest)?;
        }
    }

    Ok(())
}

fn prepare_bindgen_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    println!("  Preparing bindgen book - restoring book directory");

    let sh = Shell::new()?;
    sh.change_dir(src_dir);

    cmd!(sh, "git checkout HEAD -- book/").run()?;
    println!("  Bindgen book preparation complete");
    Ok(())
}

fn prepare_mdbook_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    let book_subdir = book.book_path.as_deref().unwrap_or("");
    let ref build_dir = if book_subdir.is_empty() {
        src_dir.to_string()
    } else {
        format!("{}/{}", src_dir, book_subdir)
    };

    println!("  Preparing mdbook book - disabling guide-helper");

    let sh = Shell::new()?;
    sh.change_dir(build_dir);

    let book_toml_path = format!("{}/book.toml", build_dir);

    cmd!(sh, "cp book.toml book.toml.backup").run()?;

    let content = fs::read_to_string(&book_toml_path)?;

    let modified_content = content.replace(
        "[preprocessor.guide-helper]",
        "# [preprocessor.guide-helper] # Disabled"
    ).replace(
        "command = \"cargo run --quiet --manifest-path guide-helper/Cargo.toml\"",
        "# command = \"cargo run --quiet --manifest-path guide-helper/Cargo.toml\""
    );

    fs::write(&book_toml_path, modified_content)?;

    println!("  mdbook book preparation complete");
    Ok(())
}

fn copy_books_to_library(books: &[Book]) -> AnyResult<()> {
    const LIBRARY_DIR: &str = "work/library";

    println!("Copying built books to library directory...");

    fs::create_dir_all(LIBRARY_DIR)?;

    let sh = Shell::new()?;
    let mut copied_count = 0;
    let mut failed_count = 0;

    for book in books {
        let book_output_dir = book_out_dir(book);
        let library_book_dir = format!("{}/{}", LIBRARY_DIR, book.slug);

        if !fs::exists(format!("{}/index.html", book_output_dir)).unwrap_or(false) {
            println!("  Skipping {} - not built successfully", book.slug);
            continue;
        }

        if fs::exists(&library_book_dir).unwrap_or(false) {
            let _ = fs::remove_dir_all(&library_book_dir);
        }

        match cmd!(sh, "cp -r {book_output_dir} {library_book_dir}").run() {
            Ok(_) => {
                println!("  Copied {} to library", book.slug);
                copied_count += 1;
            }
            Err(e) => {
                eprintln!("  Failed to copy {}: {}", book.slug, e);
                failed_count += 1;
            }
        }
    }

    println!("Library copy complete: {} copied, {} failed, {} skipped",
             copied_count, failed_count, books.len() - copied_count - failed_count);

    Ok(())
}

fn load(root: &Path) -> AnyResult<Books> {
    let path = root.join("src/books.json5");

    if let Ok(json) = std::fs::read_to_string(path) {
        return Ok(json5::from_str(&json)?);
    }

    Err(anyhow!("Could not find books.json5"))
}
