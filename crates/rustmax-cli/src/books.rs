use rmx::json5;
use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::xshell;
use rmx::xshell::{Shell, cmd};
use std::path::Path;
use std::fs;
use regex;
use toml;

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
    missing_plugins: Vec<String>,
    local_tools: Vec<LocalTool>,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
struct LocalTool {
    name: String,
    manifest_path: String,
    book_path: String,
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
    let build_results = build_books(&books, no_fetch);
    // Copy successfully built books to work/library/
    copy_books_to_library(&books)?;
    // Generate library.md with local links (only if requested)
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
    // Copy the built book to work/library/
    copy_books_to_library(&book)?;
    build_result
}

pub fn refresh_library(root: &Path) -> AnyResult<()> {
    let books = load(root)?.books;
    println!("🔄 Refreshing all library repositories...");
    for book in &books {
        if let Err(e) = get_repo(book) {
            eprintln!("Failed to refresh {}: {}", book.slug, e);
        }
    }
    println!("✅ Library refresh complete");
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
    println!("🔄 Refreshing {} repository...", slug);
    get_repo(&book[0])?;
    println!("✅ {} refresh complete", slug);
    Ok(())
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

    let mut all_local_tools: Vec<LocalTool> = results
        .iter()
        .filter(|r| !r.success)
        .flat_map(|r| r.local_tools.iter())
        .cloned()
        .collect();
    // Dedupe by name and manifest_path
    all_local_tools.sort_by(|a, b| (&a.name, &a.manifest_path).cmp(&(&b.name, &b.manifest_path)));
    all_local_tools.dedup_by(|a, b| a.name == b.name && a.manifest_path == b.manifest_path);

    if all_missing_plugins.is_empty() && all_local_tools.is_empty() {
        println!("✅ No missing dependencies detected!");
        return Ok(());
    }

    if !all_missing_plugins.is_empty() {
        println!("📦 Found {} missing published plugins:", all_missing_plugins.len());
        for plugin in &all_missing_plugins {
            println!("  - {}", plugin);
        }
    }

    if !all_local_tools.is_empty() {
        println!("🔧 Found {} local workspace tools to build:", all_local_tools.len());
        for tool in &all_local_tools {
            println!("  - {} ({})", tool.name, tool.manifest_path);
        }
    }

    if dry_run {
        if !all_missing_plugins.is_empty() {
            println!("\n🚀 Commands to install missing published plugins:");
            for plugin in &all_missing_plugins {
                println!("  cargo install {}", plugin);
            }
        }

        if !all_local_tools.is_empty() {
            println!("\n🔧 Commands to build local workspace tools:");
            for tool in &all_local_tools {
                println!("  cargo build --manifest-path {}", tool.manifest_path);
            }
        }

        println!("\nRe-run without --dry-run to install/build automatically.");
        return Ok(());
    }

    let mut failed_installs = Vec::new();
    let mut already_installed = Vec::new();
    let mut failed_builds = Vec::new();
    let mut successful_builds = Vec::new();

    // Install published plugins
    if !all_missing_plugins.is_empty() {
        println!("\n🚀 Installing missing published plugins...");

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
    }

    // Build local workspace tools
    if !all_local_tools.is_empty() {
        println!("\n🔧 Building local workspace tools...");

        for tool in &all_local_tools {
            println!("🔨 Building {} using {}...", tool.name, tool.manifest_path);
            let sh = Shell::new()?;
            let manifest_path = &tool.manifest_path;
            let result = cmd!(sh, "cargo build --manifest-path {manifest_path}").run();

            match result {
                Ok(_) => {
                    println!("  ✅ Successfully built {}", tool.name);
                    successful_builds.push(tool.name.clone());
                }
                Err(e) => {
                    eprintln!("  ❌ Failed to build {}: {}", tool.name, e);
                    failed_builds.push(tool.name.clone());
                }
            }
        }
    }

    // Print summary
    let installed_count = all_missing_plugins.len() - failed_installs.len() - already_installed.len();
    let total_success = installed_count + successful_builds.len();
    let total_failed = failed_installs.len() + failed_builds.len();

    if total_failed == 0 {
        println!("\n🎉 All dependencies resolved successfully!");
        if installed_count > 0 {
            println!("  📦 {} published plugins installed", installed_count);
        }
        if !already_installed.is_empty() {
            println!("  📝 {} published plugins were already installed", already_installed.len());
        }
        if !successful_builds.is_empty() {
            println!("  🔧 {} local tools built", successful_builds.len());
        }
    } else {
        if total_success > 0 {
            println!("\n✅ {} dependencies resolved successfully", total_success);
            if installed_count > 0 {
                println!("  📦 {} published plugins installed", installed_count);
            }
            if !successful_builds.is_empty() {
                println!("  🔧 {} local tools built", successful_builds.len());
            }
        }

        if !already_installed.is_empty() {
            println!("📝 {} published plugins were already installed", already_installed.len());
        }

        if !failed_installs.is_empty() {
            println!("\n⚠️  {} published plugins failed to install:", failed_installs.len());
            for plugin in &failed_installs {
                println!("  - {}", plugin);
            }
        }

        if !failed_builds.is_empty() {
            println!("\n⚠️  {} local tools failed to build:", failed_builds.len());
            for tool in &failed_builds {
                println!("  - {}", tool);
            }
        }

        if !failed_installs.is_empty() || !failed_builds.is_empty() {
            println!("\nYou may need to resolve these manually.");
        }
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
                local_tools: Vec::new(),
                error_message: None,
            }),
            Err((missing_plugins, local_tools, error_msg)) => results.push(BookBuildResult {
                book: book.clone(),
                success: false,
                missing_plugins,
                local_tools,
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
                    local_tools: Vec::new(),
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

        // Book-specific preparation
        if let Err(e) = prepare_book(book) {
            eprintln!("Failed to prepare {}: {}", book.slug, e);
            results.push(BookBuildResult {
                book: book.clone(),
                success: false,
                missing_plugins: Vec::new(),
                local_tools: Vec::new(),
                error_message: Some(format!("Preparation failed: {}", e)),
            });
            continue;
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
                        local_tools: Vec::new(),
                        error_message: Some(format!("Styling failed: {}", e)),
                    });
                } else {
                    results.push(BookBuildResult {
                        book: book.clone(),
                        success: true,
                        missing_plugins: Vec::new(),
                        local_tools: Vec::new(),
                        error_message: None,
                    });
                }
            }
            Err((missing_plugins, local_tools, error_msg)) => {
                if !missing_plugins.is_empty() {
                    println!("  Missing plugins for {}: {}", book.slug, missing_plugins.join(", "));
                }
                if !local_tools.is_empty() {
                    let tool_names: Vec<String> = local_tools.iter().map(|t| t.name.clone()).collect();
                    println!("  Local tools for {}: {}", book.slug, tool_names.join(", "));
                }
                eprintln!("Failed to build {}: {}", book.slug, error_msg);
                results.push(BookBuildResult {
                    book: book.clone(),
                    success: false,
                    missing_plugins,
                    local_tools,
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
    // Special case for bindgen: book source is in book/ subdir, so output goes to book-html/
    if book.slug == "bindgen" {
        format!("{BOOK_GIT_DIR}/{}/book-html", book.slug)
    } else {
        format!("{BOOK_GIT_DIR}/{}/book", book.slug)
    }
}

fn get_repo(book: &Book) -> AnyResult<()> {
    let ref repo = book.repo;
    let ref dir = book_src_dir(book);
    let ref commit = book.commit;

    // Ensure parent directory exists
    fs::create_dir_all(BOOK_GIT_DIR)?;

    let sh = Shell::new()?;
    if !fs::exists(dir)? {
        println!("  Cloning {} from {} (blobless)", book.slug, repo);
        cmd!(sh, "git clone --filter=blob:none {repo} {dir}").run()?;
    } else {
        println!("  Updating {}", book.slug);
        let _pd = sh.push_dir(dir);
        cmd!(sh, "git checkout -f").run()?;
        cmd!(sh, "git pull").run()?;
    }

    // Checkout the specific commit
    println!("  Checking out commit {} for {}", commit, book.slug);
    let _pd = sh.push_dir(dir);
    cmd!(sh, "git checkout {commit}").run()?;

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

fn build_book_with_error_detection(book: &Book) -> Result<(), (Vec<String>, Vec<LocalTool>, String)> {
    let ref src_dir = book_src_dir(book);
    let book_subdir = book.book_path.as_deref().unwrap_or("");
    let ref build_dir = if book_subdir.is_empty() {
        src_dir.to_string()
    } else {
        format!("{}/{}", src_dir, book_subdir)
    };

    // Check if the book directory exists
    if !fs::exists(build_dir).unwrap_or(false) {
        return Err((Vec::new(), Vec::new(), format!("Book directory not found: {}", build_dir)));
    }

    println!("  Building {}", book.slug);
    let sh = Shell::new().map_err(|e| (Vec::new(), Vec::new(), e.to_string()))?;
    sh.change_dir(build_dir);

    // Some books need nightly toolchain
    if book.needs_nightly || book.slug == "reference" {
        sh.set_var("RUSTUP_TOOLCHAIN", "nightly");
    }

    // Disable mdbook redirect file creation
    sh.set_var("MDBOOK_OUTPUT__HTML__REDIRECT", "{}");

    // Run mdbook build and capture output for error analysis
    let output = cmd!(sh, "mdbook build")
        .ignore_status()
        .read_stderr()
        .map_err(|e| (Vec::new(), Vec::new(), e.to_string()))?;

    // Check if build succeeded - look for index.html in the expected location
    let out_dir = if book_subdir.is_empty() {
        book_out_dir(book)
    } else {
        format!("{}/{}/book", src_dir, book_subdir)
    };

    // Some books output to book/html/ instead of book/ directly
    let index_locations = [
        format!("{}/index.html", out_dir),
        format!("{}/html/index.html", out_dir),
    ];

    let build_succeeded = index_locations.iter().any(|path| fs::exists(path).unwrap_or(false));

    if build_succeeded {
        // Build succeeded - move book if needed
        if !book_subdir.is_empty() {
            let target_dir = book_out_dir(book);
            if fs::exists(&target_dir).unwrap_or(false) {
                let _ = fs::remove_dir_all(&target_dir);
            }

            // Determine the actual source directory (might be book/ or book/html/)
            let actual_source = if fs::exists(format!("{}/html/index.html", out_dir)).unwrap_or(false) {
                format!("{}/html", out_dir)
            } else {
                out_dir.clone()
            };

            if let Err(e) = fs::rename(actual_source, target_dir) {
                return Err((Vec::new(), Vec::new(), format!("Failed to move book output: {}", e)));
            }
        }
        Ok(())
    } else {
        // Build failed - analyze error output for missing plugins and local tools
        let (missing_plugins, local_tools) = detect_missing_dependencies(&output, build_dir);
        Err((missing_plugins, local_tools, output))
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

    // Disable mdbook redirect file creation
    sh.set_var("MDBOOK_OUTPUT__HTML__REDIRECT", "{}");

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

fn detect_missing_dependencies(error_output: &str, build_dir: &str) -> (Vec<String>, Vec<LocalTool>) {
    let mut missing_plugins = Vec::new();
    let mut local_tools = Vec::new();

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

    // First, extract all missing tool names from error output
    let mut detected_tools = Vec::new();
    for (pattern, group) in plugin_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(error_output) {
                if let Some(plugin) = cap.get(group) {
                    let plugin_name = plugin.as_str();
                    // Skip common system commands that aren't mdbook plugins
                    if !["git", "cargo", "rustc", "python", "python3"].contains(&plugin_name) {
                        detected_tools.push(plugin_name.to_string());
                    }
                }
            }
        }
    }

    detected_tools.sort();
    detected_tools.dedup();

    // Parse book.toml to check for local vs published tools
    let book_toml_path = format!("{}/book.toml", build_dir);
    let local_tool_configs = parse_book_toml_for_local_tools(&book_toml_path);

    // Classify each detected tool
    for tool_name in detected_tools {
        // Check if this is a local tool defined in book.toml
        if let Some(local_config) = local_tool_configs.iter().find(|lt| {
            lt.name == tool_name || lt.name == format!("mdbook-{}", tool_name)
        }) {
            local_tools.push(local_config.clone());
        } else {
            // Convert preprocessor/backend names to plugin names for published tools
            let plugin_name = match tool_name.as_str() {
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

    missing_plugins.sort();
    missing_plugins.dedup();

    (missing_plugins, local_tools)
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
    println!("  ✅ RFCs book preparation complete");
    Ok(())
}

fn prepare_bindgen_book(book: &Book) -> AnyResult<()> {
    let ref src_dir = book_src_dir(book);
    println!("  Preparing bindgen book - restoring book directory");

    let sh = Shell::new()?;
    sh.change_dir(src_dir);

    // Try git checkout instead of git restore for older git compatibility
    cmd!(sh, "git checkout HEAD -- book/").run()?;
    println!("  ✅ Bindgen book preparation complete");
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

    println!("  Preparing mdbook book - working around guide-helper compatibility issue");

    let sh = Shell::new()?;
    sh.change_dir(build_dir);

    // The guide-helper has version compatibility issues with the installed mdbook
    // Temporarily disable it by backing up and modifying book.toml
    let book_toml_path = format!("{}/book.toml", build_dir);
    let backup_path = format!("{}/book.toml.backup", build_dir);

    // Create backup
    cmd!(sh, "cp book.toml book.toml.backup").run()?;

    // Read the current book.toml
    let content = fs::read_to_string(&book_toml_path)?;

    // Comment out the guide-helper preprocessor section
    let modified_content = content.replace(
        "[preprocessor.guide-helper]",
        "# [preprocessor.guide-helper] # Temporarily disabled due to version compatibility"
    ).replace(
        "command = \"cargo run --quiet --manifest-path guide-helper/Cargo.toml\"",
        "# command = \"cargo run --quiet --manifest-path guide-helper/Cargo.toml\""
    );

    // Write the modified content
    fs::write(&book_toml_path, modified_content)?;

    println!("  ✅ mdbook book preparation complete - guide-helper temporarily disabled");
    Ok(())
}

fn parse_book_toml_for_local_tools(book_toml_path: &str) -> Vec<LocalTool> {
    let mut local_tools = Vec::new();

    if let Ok(toml_content) = fs::read_to_string(book_toml_path) {
        if let Ok(toml_value) = toml::from_str::<toml::Value>(&toml_content) {
            // Check preprocessors section
            if let Some(preprocessors) = toml_value.get("preprocessor").and_then(|p| p.as_table()) {
                for (name, config) in preprocessors {
                    if let Some(command) = config.get("command").and_then(|c| c.as_str()) {
                        // Look for cargo run with --manifest-path patterns
                        if command.contains("cargo run") && command.contains("--manifest-path") {
                            if let Some(manifest_start) = command.find("--manifest-path") {
                                let manifest_part = &command[manifest_start + "--manifest-path".len()..].trim();
                                if let Some(manifest_path) = manifest_part.split_whitespace().next() {
                                    // Convert relative path to absolute path based on book directory
                                    let book_dir = std::path::Path::new(book_toml_path).parent().unwrap_or(std::path::Path::new("."));
                                    let absolute_manifest_path = book_dir.join(manifest_path);
                                    local_tools.push(LocalTool {
                                        name: name.clone(),
                                        manifest_path: absolute_manifest_path.to_string_lossy().to_string(),
                                        book_path: book_toml_path.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Check backends section
            if let Some(backends) = toml_value.get("output").and_then(|p| p.as_table()) {
                for (name, config) in backends {
                    if let Some(command) = config.get("command").and_then(|c| c.as_str()) {
                        // Look for cargo run with --manifest-path patterns
                        if command.contains("cargo run") && command.contains("--manifest-path") {
                            if let Some(manifest_start) = command.find("--manifest-path") {
                                let manifest_part = &command[manifest_start + "--manifest-path".len()..].trim();
                                if let Some(manifest_path) = manifest_part.split_whitespace().next() {
                                    // Convert relative path to absolute path based on book directory
                                    let book_dir = std::path::Path::new(book_toml_path).parent().unwrap_or(std::path::Path::new("."));
                                    let absolute_manifest_path = book_dir.join(manifest_path);
                                    local_tools.push(LocalTool {
                                        name: name.clone(),
                                        manifest_path: absolute_manifest_path.to_string_lossy().to_string(),
                                        book_path: book_toml_path.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    local_tools
}

fn copy_books_to_library(books: &[Book]) -> AnyResult<()> {
    const LIBRARY_DIR: &str = "work/library";

    println!("📚 Copying built books to library directory...");

    // Create the library directory
    fs::create_dir_all(LIBRARY_DIR)?;

    let sh = Shell::new()?;
    let mut copied_count = 0;
    let mut failed_count = 0;

    for book in books {
        let book_output_dir = book_out_dir(book);
        let library_book_dir = format!("{}/{}", LIBRARY_DIR, book.slug);

        // Check if the book was built successfully (has index.html)
        let index_locations = [
            format!("{}/index.html", book_output_dir),
            format!("{}/html/index.html", book_output_dir),
        ];

        let build_succeeded = index_locations.iter().any(|path| fs::exists(path).unwrap_or(false));

        if build_succeeded {
            // Determine the actual source directory (might be book/ or book/html/)
            let actual_source = if fs::exists(format!("{}/html/index.html", book_output_dir)).unwrap_or(false) {
                format!("{}/html", book_output_dir)
            } else {
                book_output_dir.clone()
            };

            // Remove existing directory if it exists
            if fs::exists(&library_book_dir).unwrap_or(false) {
                let _ = fs::remove_dir_all(&library_book_dir);
            }

            // Copy the book files
            match cmd!(sh, "cp -r {actual_source} {library_book_dir}").run() {
                Ok(_) => {
                    println!("  ✅ Copied {} to library", book.slug);
                    copied_count += 1;
                }
                Err(e) => {
                    eprintln!("  ❌ Failed to copy {}: {}", book.slug, e);
                    failed_count += 1;
                }
            }
        } else {
            println!("  ⏭️  Skipping {} - not built successfully", book.slug);
        }
    }

    println!("📚 Library copy complete: {} copied, {} failed, {} skipped",
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
