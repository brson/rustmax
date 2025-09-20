use rmx::json5;
use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::xshell;
use rmx::xshell::{Shell, cmd};
use std::path::Path;
use std::fs;
use regex;

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
}

#[derive(Debug, Clone)]
struct BookBuildResult {
    book: Book,
    success: bool,
    missing_plugins: Vec<String>,
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
    // Continue even if some books fail to build
    let _ = build_books(&books, no_fetch);
    // Generate library.md with local links (only if requested)
    if generate_library {
        crate::library_gen::generate_library_page()?;
    }
    Ok(())
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
    build_books(&book, no_fetch)
}

pub fn install_missing_plugins(root: &Path, dry_run: bool) -> AnyResult<()> {
    println!("🔍 Scanning library for missing dependencies...");

    let books = load(root)?.books;
    let results = analyze_books_for_plugins(&books);

    let mut all_missing_plugins: Vec<String> = results
        .iter()
        .filter(|r| !r.success)
        .flat_map(|r| r.missing_plugins.iter())
        .cloned()
        .collect();
    all_missing_plugins.sort();
    all_missing_plugins.dedup();

    if all_missing_plugins.is_empty() {
        println!("✅ No missing plugins detected!");
        return Ok(());
    }

    println!("📦 Found {} missing plugins:", all_missing_plugins.len());
    for plugin in &all_missing_plugins {
        println!("  - {}", plugin);
    }

    if dry_run {
        println!("\n🔧 Commands to install missing plugins:");
        for plugin in &all_missing_plugins {
            println!("  cargo install {}", plugin);
        }
        println!("\nRe-run without --dry-run to install automatically.");
        return Ok(());
    }

    println!("\n🚀 Installing missing plugins...");
    let mut failed_installs = Vec::new();
    let mut already_installed = Vec::new();

    for plugin in &all_missing_plugins {
        // Check if plugin is already installed
        let sh = Shell::new()?;
        let check_result = cmd!(sh, "cargo install --list").read();

        if let Ok(installed_list) = check_result {
            if installed_list.contains(plugin) {
                println!("  ⏭️  {} is already installed", plugin);
                already_installed.push(plugin.clone());
                continue;
            }
        }

        println!("📦 Installing {}...", plugin);
        let result = cmd!(sh, "cargo install {plugin}").run();

        match result {
            Ok(_) => println!("  ✅ Successfully installed {}", plugin),
            Err(e) => {
                eprintln!("  ❌ Failed to install {}: {}", plugin, e);
                failed_installs.push(plugin.clone());
            }
        }
    }

    let installed_count = all_missing_plugins.len() - failed_installs.len() - already_installed.len();

    if failed_installs.is_empty() && already_installed.is_empty() {
        println!("\n🎉 All {} plugins installed successfully!", installed_count);
    } else if failed_installs.is_empty() {
        println!("\n🎉 {} new plugins installed successfully!", installed_count);
        if !already_installed.is_empty() {
            println!("📝 {} plugins were already installed", already_installed.len());
        }
    } else {
        if installed_count > 0 {
            println!("\n✅ {} plugins installed successfully", installed_count);
        }
        if !already_installed.is_empty() {
            println!("📝 {} plugins were already installed", already_installed.len());
        }
        println!("\n⚠️  {} plugins failed to install:", failed_installs.len());
        for plugin in &failed_installs {
            println!("  - {}", plugin);
        }
        println!("\nYou may need to install these manually or check for different package names.");
    }

    Ok(())
}

fn analyze_books_for_plugins(books: &[Book]) -> Vec<BookBuildResult> {
    let mut results = Vec::new();

    for book in books {
        // Quick dry-run build to detect missing plugins without full processing
        let build_result = build_book_with_error_detection(book);
        match build_result {
            Ok(_) => results.push(BookBuildResult {
                book: book.clone(),
                success: true,
                missing_plugins: Vec::new(),
                error_message: None,
            }),
            Err((missing_plugins, error_msg)) => results.push(BookBuildResult {
                book: book.clone(),
                success: false,
                missing_plugins,
                error_message: Some(error_msg),
            }),
        }
    }

    results
}

fn build_books(books: &[Book], no_fetch: bool) -> AnyResult<()> {
    let mut results: Vec<BookBuildResult> = Vec::new();

    // Step 1: Clone/update repos (unless skipped)
    if !no_fetch {
        for book in books {
            println!("Processing: {} - {}", book.slug, book.name);
            if let Err(e) = get_repo(book) {
                eprintln!("Failed to clone/update {}: {}", book.slug, e);
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: false,
                    missing_plugins: Vec::new(),
                    error_message: Some(format!("Clone/update failed: {}", e)),
                });
            }
        }
    } else {
        println!("Skipping git clone/fetch operations");
    }

    // Step 2: Style hooks, build, and styling
    for book in books {
        if results.iter().any(|r| r.book.slug == book.slug && !r.success) {
            continue; // Skip books that already failed
        }

        println!("Processing: {} - {}", book.slug, book.name);

        // Style hook
        if let Err(e) = insert_style_hook(book) {
            eprintln!("Failed to insert style hook for {}: {}", book.slug, e);
        }

        // Build book - this is where we detect missing plugins
        let build_result = build_book_with_error_detection(book);
        match build_result {
            Ok(_) => {
                // Success - continue with styling
                if let Err(e) = mod_book_style(book) {
                    eprintln!("Failed to apply styling for {}: {}", book.slug, e);
                    results.push(BookBuildResult {
                        book: book.clone(),
                        success: false,
                        missing_plugins: Vec::new(),
                        error_message: Some(format!("Styling failed: {}", e)),
                    });
                } else {
                    results.push(BookBuildResult {
                        book: book.clone(),
                        success: true,
                        missing_plugins: Vec::new(),
                        error_message: None,
                    });
                }
            }
            Err((missing_plugins, error_msg)) => {
                if !missing_plugins.is_empty() {
                    println!("  Missing plugins for {}: {}", book.slug, missing_plugins.join(", "));
                }
                eprintln!("Failed to build {}: {}", book.slug, error_msg);
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: false,
                    missing_plugins,
                    error_message: Some(error_msg),
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

fn build_book_with_error_detection(book: &Book) -> Result<(), (Vec<String>, String)> {
    let ref src_dir = book_src_dir(book);
    let book_subdir = book.book_path.as_deref().unwrap_or("");
    let ref build_dir = if book_subdir.is_empty() {
        src_dir.to_string()
    } else {
        format!("{}/{}", src_dir, book_subdir)
    };

    // Check if the book directory exists
    if !fs::exists(build_dir).unwrap_or(false) {
        return Err((Vec::new(), format!("Book directory not found: {}", build_dir)));
    }

    println!("  Building {}", book.slug);
    let sh = Shell::new().map_err(|e| (Vec::new(), e.to_string()))?;
    sh.change_dir(build_dir);

    // Some books need nightly toolchain
    if book.needs_nightly || book.slug == "reference" {
        sh.set_var("RUSTUP_TOOLCHAIN", "nightly");
    }

    // Run mdbook build and capture output for error analysis
    let output = cmd!(sh, "mdbook build")
        .ignore_status()
        .read_stderr()
        .map_err(|e| (Vec::new(), e.to_string()))?;

    // Check if build succeeded
    let out_dir = if book_subdir.is_empty() {
        book_out_dir(book)
    } else {
        format!("{}/{}/book", src_dir, book_subdir)
    };

    if fs::exists(format!("{}/index.html", out_dir)).unwrap_or(false) {
        // Build succeeded - move book if needed
        if !book_subdir.is_empty() {
            let target_dir = book_out_dir(book);
            if fs::exists(&target_dir).unwrap_or(false) {
                let _ = fs::remove_dir_all(&target_dir);
            }
            if let Err(e) = fs::rename(out_dir, target_dir) {
                return Err((Vec::new(), format!("Failed to move book output: {}", e)));
            }
        }
        Ok(())
    } else {
        // Build failed - analyze error output for missing plugins
        let missing_plugins = detect_missing_plugins(&output);
        Err((missing_plugins, output))
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

fn detect_missing_plugins(error_output: &str) -> Vec<String> {
    let mut missing_plugins = Vec::new();

    // Common patterns for missing mdbook plugins
    let plugin_patterns = [
        (r#"The command `([^`]+)` wasn't found"#, 1),
        (r#"Command: ([^\s]+)"#, 1),
        (r#"missing "([^"]+)" preprocessor"#, 1),
        (r#"missing "([^"]+)" backend"#, 1),
        (r#"The "([^"]+)" preprocessor exited unsuccessfully"#, 1),
        (r#"The "([^"]+)" backend exited unsuccessfully"#, 1),
        (r#"Error: The "([^"]+)" preprocessor"#, 1),
        (r#"Error: The "([^"]+)" backend"#, 1),
    ];

    for (pattern, group) in plugin_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(error_output) {
                if let Some(plugin) = cap.get(group) {
                    let plugin_name = plugin.as_str();
                    // Skip common system commands that aren't mdbook plugins
                    if !["git", "cargo", "rustc", "python", "python3"].contains(&plugin_name) {
                        // Convert preprocessor/backend names to plugin names
                        let plugin_name = match plugin_name {
                            "guide-helper" => "mdbook-guide-helper",
                            "linkcheck" => "mdbook-linkcheck",
                            "mermaid" => "mdbook-mermaid",
                            "toc" => "mdbook-toc",
                            "trpl-listing" => "mdbook-trpl-listing",
                            "trpl-note" => "mdbook-trpl-note",
                            "gettext" => "mdbook-gettext",
                            name if name.starts_with("mdbook-") => name,
                            name => &format!("mdbook-{}", name),
                        };
                        missing_plugins.push(plugin_name.to_string());
                    }
                }
            }
        }
    }

    missing_plugins.sort();
    missing_plugins.dedup();
    missing_plugins
}

fn print_build_summary(results: &[BookBuildResult]) {
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;

    println!("\n📊 Build Summary:");
    println!("  ✅ {} books built successfully", successful);
    println!("  ❌ {} books failed", failed);

    if failed > 0 {
        println!("\n❌ Failed books:");
        for result in results.iter().filter(|r| !r.success) {
            print!("  - {}", result.book.slug);
            if !result.missing_plugins.is_empty() {
                print!(" (missing: {})", result.missing_plugins.join(", "));
            }
            println!();
        }

        // Collect all unique missing plugins
        let mut all_missing_plugins: Vec<String> = results
            .iter()
            .filter(|r| !r.success)
            .flat_map(|r| r.missing_plugins.iter())
            .cloned()
            .collect();
        all_missing_plugins.sort();
        all_missing_plugins.dedup();

        if !all_missing_plugins.is_empty() {
            println!("\n🔧 Missing plugins to install:");
            for plugin in all_missing_plugins {
                println!("  cargo install {}", plugin);
            }
        }
    }

    if successful > 0 {
        println!("\n✅ Successfully built books:");
        for result in results.iter().filter(|r| r.success) {
            println!("  - {}", result.book.slug);
        }
    }
}

fn load(root: &Path) -> AnyResult<Books> {
    let path = root.join("src/books.json5");

    if let Ok(json) = std::fs::read_to_string(path) {
        return Ok(json5::from_str(&json)?);
    }

    Err(anyhow!("Could not find books.json5"))
}
