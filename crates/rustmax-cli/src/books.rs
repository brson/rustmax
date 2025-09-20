use rmx::json5;
use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::xshell;
use rmx::xshell::{Shell, cmd};
use std::path::Path;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Books {
    books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    slug: String,
    name: String,
    repo: String,
    #[serde(default)]
    book_path: Option<String>,
    #[serde(default)]
    needs_nightly: bool,
}

pub fn list_library(root: &Path) -> AnyResult<()> {
    for book in load(root)?.books {
        println!("{}", book.slug);
    }

    Ok(())
}

pub fn build_library(root: &Path) -> AnyResult<()> {
    let books = load(root)?.books;
    // Continue even if some books fail to build
    let _ = build_books(&books);
    // Generate library.md with local links
    crate::library_gen::generate_library_page()?;
    Ok(())
}

pub fn build_one_book(root: &Path, slug: &str) -> AnyResult<()> {
    let book: Vec<Book> = load(root)?
        .books
        .into_iter()
        .filter(|b| b.slug == slug)
        .collect();
    if book.is_empty() {
        return Err(anyhow!("unknown book '{slug}'"));
    }
    build_books(&book)
}

fn build_books(books: &[Book]) -> AnyResult<()> {
    let procs = [get_repo, insert_style_hook, build_book, mod_book_style];

    let mut failed_books = Vec::new();

    for proc in procs {
        for book in books {
            println!("Processing: {} - {}", book.slug, book.name);
            if let Err(e) = proc(book) {
                eprintln!("Failed to process {}: {}", book.slug, e);
                failed_books.push(book.clone());
            }
        }
    }

    if !failed_books.is_empty() {
        eprintln!("\nFailed to build {} books:", failed_books.len());
        for book in &failed_books {
            eprintln!("  - {}", book.slug);
        }
    }

    Ok(())
}

const BOOK_GIT_DIR: &str = "work/books/";

fn book_src_dir(book: &Book) -> String {
    format!("{BOOK_GIT_DIR}/{}", book.slug)
}

fn book_out_dir(book: &Book) -> String {
    format!("{BOOK_GIT_DIR}/{}/book", book.slug)
}

fn get_repo(book: &Book) -> AnyResult<()> {
    let ref repo = book.repo;
    let ref dir = book_src_dir(book);

    // Ensure parent directory exists
    fs::create_dir_all(BOOK_GIT_DIR)?;

    let sh = Shell::new()?;
    if !fs::exists(dir)? {
        println!("  Cloning {} from {}", book.slug, repo);
        cmd!(sh, "git clone {repo} {dir}").run()?;
    } else {
        println!("  Updating {}", book.slug);
        let _pd = sh.push_dir(dir);
        cmd!(sh, "git checkout -f").run()?;
        cmd!(sh, "git pull").run()?;
    }
    Ok(())
}

fn insert_style_hook(book: &Book) -> AnyResult<()> {
    // As in book/theme/index.hbs. Goes before </head>
    let mixin_hook = r#"
        <link rel="stylesheet" href="{{ path_to_root }}mixin-mdbook-style.css">
        <script src="{{ path_to_root }}mixin-mdbook-script.js"></script>
"#;

    let ref src_dir = book_src_dir(book);
    let theme_dir = format!("{src_dir}/theme");
    let ref index_file = format!("{theme_dir}/index.hbs");

    // Create theme directory if it doesn't exist
    if !fs::exists(&theme_dir)? {
        fs::create_dir_all(&theme_dir)?;
    }

    if fs::exists(index_file)? {
        let mut index = fs::read_to_string(index_file)?;

        // Only insert if not already present
        if !index.contains("mixin-mdbook-style.css") {
            if let Some(insert_point) = index.find("</head>") {
                index.insert_str(insert_point, mixin_hook);
                fs::write(index_file, &index)?;
            }
        }
    } else {
        // Copy our template index.hbs
        let rustmax_index_hbs = "book/theme/index.hbs";
        if fs::exists(rustmax_index_hbs)? {
            fs::copy(rustmax_index_hbs, index_file)?;
        }
    }

    Ok(())
}

fn build_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    let book_subdir = book.book_path.as_deref().unwrap_or("");
    let ref build_dir = if book_subdir.is_empty() {
        src_dir.to_string()
    } else {
        format!("{}/{}", src_dir, book_subdir)
    };

    // Check if the book directory exists
    if !fs::exists(build_dir)? {
        eprintln!("  Book directory not found: {}", build_dir);
        return Err(anyhow!("Book directory not found"));
    }

    println!("  Building {}", book.slug);
    let sh = Shell::new()?;
    sh.change_dir(build_dir);

    // Some books need nightly toolchain
    if book.needs_nightly || book.slug == "reference" {
        sh.set_var("RUSTUP_TOOLCHAIN", "nightly");
    }

    cmd!(sh, "mdbook build").run()?;

    // Verify the build succeeded
    let out_dir = if book_subdir.is_empty() {
        book_out_dir(book)
    } else {
        format!("{}/{}/book", src_dir, book_subdir)
    };

    if !fs::exists(format!("{}/index.html", out_dir))? {
        return Err(anyhow!("Build failed - index.html not found"));
    }

    // If book was built in a subdirectory, move it to the expected location
    if !book_subdir.is_empty() {
        let target_dir = book_out_dir(book);
        if fs::exists(&target_dir)? {
            fs::remove_dir_all(&target_dir)?;
        }
        fs::rename(out_dir, target_dir)?;
    }

    Ok(())
}

fn mod_book_style(book: &Book) -> AnyResult<()> {
    // same as in justfile doc-book
    let mixins = [
        ("www/mixins", "mixin-mdbook-style.css"),
        ("www/mixins", "mixin-mdbook-script.js"),
        ("www", "rustmax-themes.css"),
    ];

    let ref out_dir = book_out_dir(book);
    assert!(fs::exists(out_dir)?);

    for (src_dir, file) in mixins {
        fs::copy(format!("{src_dir}/{file}"), format!("{out_dir}/{file}"))?;
    }

    Ok(())
}

fn load(root: &Path) -> AnyResult<Books> {
    let path = root.join("src/books.json5");

    if let Ok(json) = std::fs::read_to_string(path) {
        return Ok(json5::from_str(&json)?);
    }

    Err(anyhow!("Could not find books.json5"))
}
