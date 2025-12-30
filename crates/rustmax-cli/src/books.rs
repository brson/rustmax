use rmx::json5;
use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::xshell::{Shell, cmd};
use std::path::Path;
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
    let build_results = build_books(&books, no_fetch);
    copy_books_to_library(&books)?;
    if generate_library {
        crate::library_gen::generate_library_page()?;
    }
    build_results
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
    let build_result = build_books(&book, no_fetch);
    copy_books_to_library(&book)?;
    build_result
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

fn build_books(books: &[Book], no_fetch: bool) -> AnyResult<()> {
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

fn prepare_rfcs_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    println!("  Preparing RFCs book - generating SUMMARY.md");

    let sh = Shell::new()?;
    sh.change_dir(src_dir);

    cmd!(sh, "python3 generate-book.py").run()?;
    println!("  RFCs book preparation complete");
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
