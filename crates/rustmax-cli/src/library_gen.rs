use rmx::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

pub fn generate_library_page() -> AnyResult<()> {
    let root = std::path::Path::new(".");
    let books = load_books(root)?;
    let content = generate_markdown(&books)?;
    fs::write("book/src/library.md", content)?;
    println!("Generated library.md with local book links");
    Ok(())
}

fn load_books(root: &std::path::Path) -> AnyResult<Books> {
    let path = root.join("src/books.json5");
    let json = fs::read_to_string(path)?;
    Ok(rmx::json5::from_str(&json)?)
}

fn generate_markdown(books: &Books) -> AnyResult<String> {
    let mut content = String::new();

    // Header
    content.push_str("# The Rust Max Library\n\n");
    content.push_str("The Rust language and its ecosystem is documented in \"books\"\n");
    content.push_str("(rendered with [`mdbook`]), and most of these links are to books.\n\n");
    content.push_str("Links with a bookmark icon, 🔖, are to particularly\n");
    content.push_str("notable or useful chapters within a book.\n\n");

    // The Rust language section
    content.push_str("## The Rust language\n\n");

    // Map book slugs to their entries
    let book_map: std::collections::HashMap<&str, &Book> = books
        .books
        .iter()
        .map(|b| (b.slug.as_str(), b))
        .collect();

    // Core language books
    if let Some(book) = book_map.get("trpl") {
        content.push_str(&format!(
            "- **[{}](/books/{}/)** ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("rust-by-example") {
        content.push_str(&format!(
            "- **[{}](/books/{}/)** ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("reference") {
        content.push_str(&format!(
            "- **[{}](/books/{}/)** ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
        content.push_str("  - 🔖 [Conditional compilation](/books/reference/conditional-compilation.html).\n");
        content.push_str("       Including which cfgs are set by rustc.\n");
        content.push_str("  - 🔖 [Behavior considered undefined](/books/reference/behavior-considered-undefined.html)\n");
    }
    if let Some(book) = book_map.get("nomicon") {
        content.push_str(&format!(
            "- **[{}](/books/{}/)** ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("edition-guide") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    // External books (not in our build)
    content.push_str("- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)\n");

    if let Some(book) = book_map.get("api-guidelines") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("unsafe-code-guidelines") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    // External resources
    content.push_str("- [Rust Error Codes Index](https://doc.rust-lang.org/stable/error_codes/error-index.html)\n");
    content.push_str("- [The Rust Unstable Book](https://doc.rust-lang.org/unstable-book/)\n");
    content.push_str("- [The Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/index.html)\n");
    content.push_str("- [Rust Release Notes](https://doc.rust-lang.org/nightly/releases.html)\n");

    // The Rust standard library section
    content.push_str("\n## The Rust standard library\n\n");
    content.push_str("- **[`std`](https://doc.rust-lang.org/std/index.html)**\n");
    content.push_str("  <!-- duplicated in std.md -->\n");
    content.push_str("  - 🔖 [`std::collections`](https://doc.rust-lang.org/std/collections/index.html)\n");
    content.push_str("  - 🔖 [`std::error`](https://doc.rust-lang.org/stable/std/error/index.html)\n");
    content.push_str("  - 🔖 [`std::ptr`](https://doc.rust-lang.org/std/ptr/index.html).\n");
    content.push_str("    Safety, undefined behavior, and \"provenance\".\n");
    content.push_str("  - 🔖 [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)\n");
    content.push_str("  - 🔖 [`std::sync::atomic`](https://doc.rust-lang.org/std/atomic/index.html)\n");
    content.push_str("- [`core`](https://doc.rust-lang.org/core/index.html)\n");
    content.push_str("- [`alloc`](https://doc.rust-lang.org/alloc/index.html)\n");
    content.push_str("- [`proc_macro`](https://doc.rust-lang.org/proc_macro/index.html)\n");

    // Standard Rust tools section
    content.push_str("\n\n## Standard Rust tools\n\n");
    content.push_str("<!-- order here is same is in tools.md -->\n");

    if let Some(book) = book_map.get("cargo-book") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
        content.push_str("  - 🔖 [The manifest format](/books/cargo-book/reference/manifest.html)\n");
        content.push_str("  - 🔖 [Environment variables](/books/cargo-book/reference/environment-variables.html)\n");
        content.push_str("    that affect the Cargo build process.\n");
        content.push_str("  - 🔖 [Configuration format](/books/cargo-book/reference/config.html).\n");
        content.push_str("    Cargo has many interesting configuration options.\n");
    }
    if let Some(book) = book_map.get("rustc-book") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
        content.push_str("  - 🔖 [The lint system and built-in lints](/books/rustc-book/lints/index.html)\n");
        content.push_str("  - 🔖 [Rust platform support tiers](/books/rustc-book/platform-support.html)\n");
    }

    // External tool docs
    content.push_str("- [The `rustup` Book](https://rust-lang.github.io/rustup/index.html)\n");

    if let Some(book) = book_map.get("rustdoc-book") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    content.push_str("- rustfmt (todo)\n");
    content.push_str("- [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/development/infrastructure/book.html)\n");
    content.push_str("- [The `just` Programmer's Manual](https://just.systems/man/en/)\n");

    if let Some(book) = book_map.get("mdbook") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("bindgen") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    content.push_str("- miri (todo)\n");

    // The Rust crate ecosystem section
    content.push_str("\n## The Rust crate ecosystem\n\n");

    if let Some(book) = book_map.get("rand-book") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    content.push_str("- [The `proptest` Book](https://proptest-rs.github.io/proptest/intro.html)\n");
    content.push_str("- [The `serde` Book](https://serde.rs/)\n");

    if let Some(book) = book_map.get("rust-cookbook") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    // Domain-specific Rust section
    content.push_str("\n## Domain-specific Rust\n\n");

    if let Some(book) = book_map.get("embedded-book") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    // The Rust Project internals section
    content.push_str("\n## The Rust Project internals\n\n");
    content.push_str("- [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/)\n");

    if let Some(book) = book_map.get("rustc-dev-guide") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("std-dev-guide") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }
    if let Some(book) = book_map.get("rust-forge") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
        content.push_str("  - 🔖 [Alternative Rust Installation Methods](/books/rust-forge/infra/other-installation-methods.html)\n");
    }
    if let Some(book) = book_map.get("rfcs") {
        content.push_str(&format!(
            "- [{}](/books/{}/) ([source]({}))\n",
            book.name, book.slug, book.repo
        ));
    }

    // Footer
    content.push_str("\n\n\n[`mdbook`]: https://github.com/rust-lang/mdBook\n");

    Ok(content)
}