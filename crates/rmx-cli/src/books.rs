use rmx::prelude::*;
use rmx::serde::{Serialize, Deserialize};
use rmx::xshell;
use rmx::json5;
use rmx::xshell::{Shell, cmd};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Books {
    books: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
struct Book {
    slug: String,
    name: String,
    repo: String,
}

pub fn list_library() -> AnyResult<()> {
    for book in load()?.books {
        println!("{}", book.slug);
    }

    Ok(())
}

pub fn build_library() -> AnyResult<()> {
    build_books(&load()?.books)
}

pub fn build_one_book(slug: &str) -> AnyResult<()> {
    let book: Vec<Book> = load()?.books
        .into_iter()
        .filter(|b| b.slug == slug)
        .collect();
    if book.is_empty() {
        return Err(anyhow!("unknown book '{slug}'"));
    }
    build_books(&book)
}

fn build_books(books: &[Book]) -> AnyResult<()> {
    let procs = [
        get_repo,
        build_book,
    ];

    for proc in procs {
        for book in books {
            proc(book)?;
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
    let sh = Shell::new()?;
    if !fs::exists(dir)? {
        cmd!(sh, "git clone {repo} {dir}").run()?;
    } else {
        let _pd = sh.push_dir(dir);
        cmd!(sh, "git pull").run()?;
    }
    Ok(())
}

fn build_book(book: &Book) -> AnyResult<()> {
    let ref dir = book_src_dir(book);
    assert!(fs::exists(dir)?);
    let sh = Shell::new()?;
    sh.change_dir(dir);
    sh.set_var("RUSTUP_TOOLCHAIN", "nightly"); // fixme - rust reference needs nightly
    cmd!(sh, "mdbook build").run()?;
    assert!(fs::exists(format!("{}/index.html", book_out_dir(book)))?);
    Ok(())
}

fn load() -> AnyResult<Books> {
    let path = "src/books.json5";
    let json = std::fs::read_to_string(path)?;
    Ok(json5::from_str(&json)?)
}
